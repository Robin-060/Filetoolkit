use blake3::Hasher;
use rayon::prelude::*;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use tauri::{AppHandle, command, Emitter};
use trash::delete;
use walkdir::WalkDir;

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct FileInfo {
    pub path: String,
    pub size: u64,
    pub modified: u64,
    pub hash: String,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct DuplicateGroup {
    pub files: Vec<FileInfo>,
}

fn quick_hash(file_path: &Path) -> Result<String, String> {
    let mut file = std::fs::File::open(file_path).map_err(|e| e.to_string())?;
    let mut buf = [0u8; 8192];
    let read_len = std::io::Read::read(&mut file, &mut buf).map_err(|e| e.to_string())?;
    let mut hasher = Hasher::new();
    hasher.update(&buf[0..read_len]);
    Ok(hasher.finalize().to_hex().to_string())
}

fn full_hash(file_path: &Path) -> Result<String, String> {
    let mut file = std::fs::File::open(file_path).map_err(|e| e.to_string())?;
    let mut hasher = Hasher::new();
    std::io::copy(&mut file, &mut hasher).map_err(|e| e.to_string())?;
    Ok(hasher.finalize().to_hex().to_string())
}

fn get_modify_ts(path: &Path) -> Result<u64, String> {
    let meta = std::fs::metadata(path).map_err(|e| e.to_string())?;
    let mtime = meta.modified().map_err(|e| e.to_string())?;
    let ts = mtime
        .duration_since(std::time::SystemTime::UNIX_EPOCH)
        .map_err(|e| e.to_string())?
        .as_millis() as u64;
    Ok(ts)
}

#[command]
pub async fn scan_duplicates(app: AppHandle, dir: String) -> Result<Vec<DuplicateGroup>, String> {
    let dir_path = Path::new(&dir);
    if !dir_path.is_dir() {
        return Err("传入路径不是有效文件夹".to_string());
    }

    let mut size_groups: HashMap<u64, Vec<PathBuf>> = HashMap::new();
    let all_entries = WalkDir::new(dir_path).into_iter().filter_map(|e| e.ok());
    for entry in all_entries {
        let path = entry.path();
        if !path.is_file() {
            continue;
        }
        let size = std::fs::metadata(path)
            .map_err(|e| format!("读取文件{}元数据失败:{}", path.display(), e))?
            .len();
        size_groups.entry(size).or_default().push(path.to_path_buf());
    }

    let candidate_files: Vec<PathBuf> = size_groups
        .into_iter()
        .filter(|(_, paths)| paths.len() > 1)
        .flat_map(|(_, paths)| paths)
        .collect();

    let total = candidate_files.len();
    let app_arc = Arc::new(app);
    let hash_map = Arc::new(Mutex::new(HashMap::<String, Vec<FileInfo>>::new()));

    candidate_files
        .into_par_iter()
        .enumerate()
        .for_each(|(idx, file_path)| {
            let progress = ((idx + 1) as f64 / total as f64) * 100.0;
            let _ = app_arc.emit("dedup_scan_progress", progress);

            let _quick_h = match quick_hash(&file_path) {
                Ok(h) => h,
                Err(_) => return,
            };
            let full_h = match full_hash(&file_path) {
                Ok(h) => h,
                Err(_) => return,
            };
            let size = std::fs::metadata(&file_path).unwrap().len();
            let modified = get_modify_ts(&file_path).unwrap_or(0);
            let file_info = FileInfo {
                path: file_path.to_str().unwrap().to_string(),
                size,
                modified,
                hash: full_h.clone(),
            };
            hash_map.lock().unwrap().entry(full_h).or_default().push(file_info);
        });

    let hash_map = Arc::into_inner(hash_map).unwrap().into_inner().unwrap();
    let mut result_groups = Vec::new();
    for files in hash_map.into_values() {
        if files.len() >= 2 {
            result_groups.push(DuplicateGroup { files });
        }
    }
    Ok(result_groups)
}

#[command]
pub async fn delete_duplicates(
    keep_strategy: String,
    groups: Vec<DuplicateGroup>,
) -> Result<(), String> {
    for group in groups {
        let mut files = group.files;
        if files.len() <= 1 {
            continue;
        }
        let to_remove: Vec<FileInfo> = match keep_strategy.as_str() {
            "newest" => {
                let newest_idx = files
                    .iter()
                    .enumerate()
                    .max_by_key(|(_, f)| f.modified)
                    .map(|(i, _)| i)
                    .unwrap();
                files.remove(newest_idx);
                files
            }
            "first" => {
                files.remove(0);
                files
            }
            "largest" => {
                let largest_idx = files
                    .iter()
                    .enumerate()
                    .max_by_key(|(_, f)| f.size)
                    .map(|(i, _)| i)
                    .unwrap();
                files.remove(largest_idx);
                files
            }
            _ => return Err("不支持的保留策略，仅支持 newest / largest / first".to_string()),
        };
        for file in to_remove {
            let path = Path::new(&file.path);
            delete(path).map_err(|e| format!("删除文件{}失败:{}", file.path, e))?;
        }
    }
    Ok(())
}
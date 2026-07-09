use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use serde::{Serialize, Deserialize};
use tauri::command;

#[derive(Serialize, Deserialize, Debug)]
pub struct FileInfo {
    pub name: String,
    pub size: u64,
    pub path: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DuplicateGroup {
    pub hash: String,
    pub files: Vec<FileInfo>,
}

#[command]
pub fn scan_duplicate_files(folder_path: String) -> Result<Vec<DuplicateGroup>, String> {
    let mut size_map: HashMap<u64, Vec<FileInfo>> = HashMap::new();
    let entries = fs::read_dir(&folder_path).map_err(|e| e.to_string())?;

    for entry in entries {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();
        if !path.is_file() {
            continue;
        }
        let meta = fs::metadata(&path).map_err(|e| e.to_string())?;
        let file_size = meta.len();
        let file_info = FileInfo {
            name: path.file_name().unwrap().to_string_lossy().to_string(),
            size: file_size,
            path: path.to_string_lossy().to_string(),
        };
        size_map.entry(file_size).or_default().push(file_info);
    }

    let mut dup_groups = Vec::new();
    for (size, file_list) in size_map {
        if file_list.len() >= 2 {
            dup_groups.push(DuplicateGroup {
                hash: format!("size_{}", size),
                files: file_list,
            });
        }
    }
    Ok(dup_groups)
}

#[command]
pub fn delete_duplicate_file(file_path: String) -> Result<(), String> {
    fs::remove_file(file_path).map_err(|e| e.to_string())
}
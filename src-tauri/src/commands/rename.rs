use std::fs;
use std::path::{Path, PathBuf};
use tauri::command;

#[command]
pub fn rename_file(folder_path: String, prefix: String, start_num: u32) -> Result<(), String> {
    let dir_path = Path::new(&folder_path);
    if !dir_path.is_dir() {
        return Err("目标文件夹不存在".to_string());
    }

    let entries = fs::read_dir(dir_path).map_err(|e| e.to_string())?;
    let mut file_list: Vec<PathBuf> = Vec::new();

    for entry in entries {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();
        if path.is_file() {
            file_list.push(path);
        }
    }

    let mut current = start_num;
    for file in file_list {
        let ext = file
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("");
        let new_file_name = format!("{}{}.{}", prefix, current, ext);
        let new_path = dir_path.join(new_file_name);

        fs::rename(&file, &new_path).map_err(|e| e.to_string())?;
        current += 1;
    }

    Ok(())
}
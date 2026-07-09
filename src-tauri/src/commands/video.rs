use std::fs;
use std::path::PathBuf;
use tauri::command;

#[command]
pub fn get_video_files(folder_path: String) -> Result<Vec<PathBuf>, String> {
    let mut result = Vec::new();
    let entries = fs::read_dir(folder_path).map_err(|e| e.to_string())?;
    for entry in entries {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();
        if path.is_file() {
            if let Some(ext) = path.extension() {
                match ext.to_str() {
                    Some("mp4") | Some("mov") | Some("avi") | Some("mkv") => {
                        result.push(path);
                    }
                    _ => {}
                }
            }
        }
    }
    Ok(result)
}
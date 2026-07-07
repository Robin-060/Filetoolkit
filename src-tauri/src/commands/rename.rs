// 批量重命名命令(M1 阶段填充)。
// 功能规划见 docs/PRD.md §3.2.5:
//   - 模板变量:{name} {ext} {date} {index:3} {exif.date}
//   - 实时预览、冲突检测
//   - 支持 Exif 拍摄日期、文件创建/修改时间作为命名来源
use chrono::prelude::*;
use std::collections::HashSet;
use std::path::Path;
use std::result::Result;
use tauri::{AppHandle, command, Emitter};
use trash::delete;

#[derive(serde::Serialize, Clone, Debug)]
pub struct RenamePrev {
    pub old_name: String,
    pub new_name: String,
    pub conflict: bool,
}

pub fn parse_pattern(
    pattern: &str,
    file_path: &Path,
    index: usize,
) -> Result<String, String> {
    let stem = file_path
        .file_stem()
        .and_then(|s| s.to_str())
        .ok_or("文件名非法")?;
    let ext = file_path
        .extension()
        .and_then(|s| s.to_str())
        .map(|e| format!(".{}", e))
        .unwrap_or_default();

    let meta = std::fs::metadata(file_path).map_err(|e| e.to_string())?;
    let mtime: DateTime<Local> = meta.modified().map_err(|e| e.to_string())?.into();

    let mut res = pattern.to_string();
    res = res.replace("{name}", stem);
    res = res.replace("{ext}", &ext);
    res = res.replace("{index:3}", &format!("{:03}", index));
    res = res.replace("{index}", &index.to_string());
    res = res.replace("{date:yyyy-MM-dd}", &mtime.format("%Y-%m-%d").to_string());
    res = res.replace("{date}", &mtime.format("%Y-%m-%d").to_string());

    Ok(res)
}

#[command]
pub fn preview_rename(files: Vec<String>, pattern: String) -> Result<Vec<RenamePrev>, String> {
    let mut preview_list = Vec::new();
    let mut new_name_set = HashSet::new();

    for (idx, file_abs) in files.iter().enumerate() {
        let path = Path::new(file_abs);
        let old_name = path
            .file_name()
            .and_then(|s| s.to_str())
            .ok_or("文件路径错误")?
            .to_string();

        let new_name = parse_pattern(&pattern, path, idx)?;
        let conflict = new_name_set.contains(&new_name);
        new_name_set.insert(new_name.clone());

        preview_list.push(RenamePrev {
            old_name,
            new_name,
            conflict,
        });
    }

    Ok(preview_list)
}

#[command]
pub async fn execute_rename(
    app: AppHandle,
    plan: Vec<(String, String)>,
) -> Result<(), String> {
    let total = plan.len();
    for (cur, (old_path, new_name)) in plan.into_iter().enumerate() {
        app.emit("rename_progress", (cur + 1, total)).map_err(|e| e.to_string())?;

        let old_p = Path::new(&old_path);
        let parent = old_p.parent().ok_or("目录获取失败")?;
        let new_p = parent.join(new_name);

        delete(old_p).map_err(|e| e.to_string())?;
        std::fs::rename(old_p, new_p).map_err(|e| e.to_string())?;
    }
    Ok(())
}
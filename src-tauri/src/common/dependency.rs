// 可选外部依赖管理 —— ffmpeg / LibreOffice / Tesseract 的探测、缓存与引导安装。
// 核心思想:这些工具不打包进安装包,首次使用时检测→缺失则引导下载→结果缓存。

use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::process::Command;

// ============================================================
// 数据结构
// ============================================================

/// 依赖状态,通过 Tauri 命令返回给前端。
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DependencyStatus {
    /// 依赖名称,如 "ffmpeg"
    pub name: String,
    /// 是否已安装
    pub found: bool,
    /// 可执行文件路径(如果找到)
    pub path: Option<String>,
    /// 版本号(如果找到)
    pub version: Option<String>,
    /// 安装引导文案(如果未找到)
    pub guidance: String,
}

/// 依赖定义:名称、可执行文件名、常见安装路径。
#[derive(Clone)]
pub struct DependencyInfo {
    pub name: &'static str,
    pub exe_name: &'static str,
    pub common_paths: &'static [&'static str],
    pub download_url: &'static str,
    pub install_guide: &'static str,
}

// ============================================================
// 依赖注册表
// ============================================================

const KNOWN_DEPENDENCIES: &[DependencyInfo] = &[
    DependencyInfo {
        name: "ffmpeg",
        exe_name: if cfg!(windows) { "ffmpeg.exe" } else { "ffmpeg" },
        common_paths: if cfg!(windows) {
            &[
                // winget / scoop / chocolatey 默认路径
                "C:\\Program Files\\ffmpeg\\bin",
                "C:\\ffmpeg\\bin",
                // scoop
                "%USERPROFILE%\\scoop\\apps\\ffmpeg\\current\\bin",
                // 本应用内置路径
                "%LOCALAPPDATA%\\file-toolkit\\ffmpeg\\bin",
            ]
        } else if cfg!(target_os = "macos") {
            &[
                "/usr/local/bin",
                "/opt/homebrew/bin",
                "~/.local/bin",
            ]
        } else {
            &[
                "/usr/bin",
                "/usr/local/bin",
                "~/.local/bin",
            ]
        },
        download_url: "https://ffmpeg.org/download.html",
        install_guide: if cfg!(windows) {
            "请从 https://www.gyan.dev/ffmpeg/builds/ 下载 essentials build,解压后将 bin 目录加入 PATH,或放到 %LOCALAPPDATA%\\file-toolkit\\ffmpeg\\"
        } else if cfg!(target_os = "macos") {
            "请运行: brew install ffmpeg"
        } else {
            "请运行: sudo apt install ffmpeg 或从 ffmpeg.org 下载"
        },
    },
    DependencyInfo {
        name: "libreoffice",
        exe_name: if cfg!(windows) { "soffice.exe" } else { "soffice" },
        common_paths: if cfg!(windows) {
            &["C:\\Program Files\\LibreOffice\\program"]
        } else if cfg!(target_os = "macos") {
            &["/Applications/LibreOffice.app/Contents/MacOS"]
        } else {
            &["/usr/bin", "/usr/local/bin"]
        },
        download_url: "https://www.libreoffice.org/download/",
        install_guide: "请从 libreoffice.org 下载并安装 LibreOffice。",
    },
    DependencyInfo {
        name: "tesseract",
        exe_name: if cfg!(windows) { "tesseract.exe" } else { "tesseract" },
        common_paths: if cfg!(windows) {
            &["C:\\Program Files\\Tesseract-OCR"]
        } else {
            &["/usr/bin", "/usr/local/bin", "/opt/homebrew/bin"]
        },
        download_url: "https://github.com/UB-Mannheim/tesseract/wiki",
        install_guide: "请安装 Tesseract OCR,并下载中英文语言包(chi_sim, eng)。",
    },
];

// ============================================================
// 缓存
// ============================================================

fn cache_dir() -> Option<PathBuf> {
    dirs::cache_dir().map(|d| d.join("file-toolkit"))
}

fn cache_file(dep_name: &str) -> Option<PathBuf> {
    cache_dir().map(|d| d.join(format!("dep-{}.json", dep_name)))
}

fn read_cache(dep_name: &str) -> Option<DependencyStatus> {
    let file = cache_file(dep_name)?;
    if file.exists() {
        let text = std::fs::read_to_string(&file).ok()?;
        serde_json::from_str(&text).ok()
    } else {
        None
    }
}

fn write_cache(status: &DependencyStatus) {
    if let Some(file) = cache_file(&status.name) {
        if let Some(parent) = file.parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        if let Ok(json) = serde_json::to_string(status) {
            let _ = std::fs::write(&file, json);
        }
    }
}

// ============================================================
// 探测逻辑
// ============================================================

/// 在 PATH 中搜索可执行文件。
fn find_in_path(exe_name: &str) -> Option<PathBuf> {
    if let Ok(path_var) = std::env::var("PATH") {
        for dir in path_var.split(if cfg!(windows) { ';' } else { ':' }) {
            let candidate = PathBuf::from(dir).join(exe_name);
            if candidate.is_file() {
                return Some(candidate);
            }
        }
    }
    None
}

/// 在常见安装路径中搜索。
fn find_in_common_paths(exe_name: &str, common_paths: &[&str]) -> Option<PathBuf> {
    for path_template in common_paths {
        // 展开环境变量(如 %USERPROFILE%)
        let expanded = expand_env(path_template);
        let candidate = PathBuf::from(&expanded).join(exe_name);
        if candidate.is_file() {
            return Some(candidate);
        }
    }
    None
}

fn expand_env(s: &str) -> String {
    // 简单的 %VAR% 展开
    let mut result = s.to_string();
    if cfg!(windows) {
        // 处理 %VAR% 格式:逐个查找并展开
        let mut i = 0;
        while let Some(start) = result[i..].find('%') {
            let abs_start = i + start;
            if let Some(end) = result[abs_start + 1..].find('%') {
                let abs_end = abs_start + 1 + end;
                let var_name = &result[abs_start + 1..abs_end];
                let value = std::env::var(var_name).unwrap_or_default();
                let before = result[..abs_start].to_string();
                let after = result[abs_end + 1..].to_string();
                result = format!("{}{}{}", before, value, after);
                i = before.len() + value.len();
            } else {
                break;
            }
        }
    } else {
        // 处理 $VAR 和 ${VAR} 格式
        // 简化为展开 ~
        if let Some(home) = dirs::home_dir() {
            result = result.replace("~/", &format!("{}/", home.display()));
        }
    }
    result
}

/// 获取版本号。
fn get_version(exe_path: &Path, dep: &DependencyInfo) -> Option<String> {
    let output = match dep.name {
        "ffmpeg" => Command::new(exe_path).arg("-version").output().ok(),
        "libreoffice" => Command::new(exe_path).arg("--version").output().ok(),
        "tesseract" => Command::new(exe_path).arg("--version").output().ok(),
        _ => return None,
    }?;
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    let combined = format!("{}{}", stdout, stderr);
    // 取第一行作为版本号
    combined.lines().next().map(|s| s.to_string())
}

/// 探测单个依赖(先查缓存,再查 PATH,再查常见路径)。
pub fn detect_dependency(dep: &DependencyInfo) -> DependencyStatus {
    // 1. 缓存命中
    if let Some(cached) = read_cache(dep.name) {
        if cached.found {
            // 验证缓存的路径仍然有效
            if let Some(ref path) = cached.path {
                if Path::new(path).exists() {
                    return cached;
                }
            }
        }
    }

    // 2. 搜索 PATH
    if let Some(exe_path) = find_in_path(dep.exe_name) {
        let status = DependencyStatus {
            name: dep.name.to_string(),
            found: true,
            path: Some(exe_path.display().to_string()),
            version: get_version(&exe_path, dep),
            guidance: String::new(),
        };
        write_cache(&status);
        return status;
    }

    // 3. 搜索常见路径
    if let Some(exe_path) = find_in_common_paths(dep.exe_name, dep.common_paths) {
        let status = DependencyStatus {
            name: dep.name.to_string(),
            found: true,
            path: Some(exe_path.display().to_string()),
            version: get_version(&exe_path, dep),
            guidance: String::new(),
        };
        write_cache(&status);
        return status;
    }

    // 4. 未找到
    let status = DependencyStatus {
        name: dep.name.to_string(),
        found: false,
        path: None,
        version: None,
        guidance: format!(
            "{}\n下载地址: {}",
            dep.install_guide, dep.download_url
        ),
    };
    write_cache(&status);
    status
}

/// 查找指定依赖的可执行文件路径,未找到时返回 AppError。
/// 供其他命令调用(如视频处理前先确保 ffmpeg 存在)。
pub fn require_dependency(dep_name: &str) -> crate::common::error::AppResult<PathBuf> {
    let dep = KNOWN_DEPENDENCIES
        .iter()
        .find(|d| d.name == dep_name)
        .ok_or_else(|| crate::common::error::AppError::DependencyNotFound(dep_name.to_string()))?;

    let status = detect_dependency(dep);
    if status.found {
        Ok(PathBuf::from(status.path.unwrap()))
    } else {
        Err(crate::common::error::AppError::DependencyNotFound(
            status.guidance,
        ))
    }
}

// ============================================================
// Tauri 命令
// ============================================================

/// 检查指定依赖是否可用。
/// 前端启动时调用此命令,显示依赖状态面板。
#[tauri::command]
pub fn check_dependency(name: String) -> Result<DependencyStatus, String> {
    let dep = KNOWN_DEPENDENCIES
        .iter()
        .find(|d| d.name == name.as_str())
        .ok_or_else(|| format!("未知依赖: {}", name))?;

    Ok(detect_dependency(dep))
}

/// 获取全部已知依赖的状态(用于设置页面的依赖管理面板)。
#[tauri::command]
pub fn check_all_dependencies() -> Vec<DependencyStatus> {
    KNOWN_DEPENDENCIES
        .iter()
        .map(|dep| detect_dependency(dep))
        .collect()
}

/// 清除依赖缓存,强制下次重新探测。
#[tauri::command]
pub fn clear_dependency_cache() {
    if let Some(dir) = cache_dir() {
        if let Ok(entries) = std::fs::read_dir(&dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path
                    .file_name()
                    .and_then(|n| n.to_str())
                    .map(|n| n.starts_with("dep-"))
                    .unwrap_or(false)
                {
                    let _ = std::fs::remove_file(&path);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expand_env_windows() {
        if cfg!(windows) {
            let result = expand_env("%USERPROFILE%\\test");
            assert!(!result.contains('%'), "环境变量应被展开: {}", result);
        }
    }

    #[test]
    fn test_detect_ffmpeg_no_panic() {
        let dep = KNOWN_DEPENDENCIES.iter().find(|d| d.name == "ffmpeg").unwrap();
        let status = detect_dependency(dep);
        // 无论是否找到,都不应 panic
        assert_eq!(status.name, "ffmpeg");
    }
}

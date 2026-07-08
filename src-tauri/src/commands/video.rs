<<<<<<< HEAD
// 视频处理命令(M2 阶段填充)。
// 功能规划见 docs/PRD.md §3.2.3:
//   - 剪切 / 裁剪、格式转码、批量压缩(H.265)
//   - 音频提取 / GIF 制作、字幕烧录 / 提取
//   - 硬件加速(NVENC / QSV / VideoToolbox)
// 底层统一调用 ffmpeg 子进程。
=======
// 视频处理命令(角色 A · M2 阶段)
//
// 功能:剪切 / 转码 / GPU 检测 / GIF 生成 / 音频提取
// 底层统一调用 ffmpeg 子进程,通过 tokio::spawn_blocking 避免阻塞主线程。

use std::path::{Path, PathBuf};
use std::process::Stdio;
use tauri::{AppHandle, command, Emitter};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct VideoCompressRequest {
    pub input_path: String,
    pub output_path: String,
    pub quality: String,
    pub resolution: Option<String>,
    pub fps: Option<u32>,
}

#[derive(Debug, Serialize)]
pub struct VideoCompressProgress {
    pub percent: f64,
    pub speed: String,
    pub eta: String,
}

#[derive(Debug, Serialize)]
pub struct VideoCompressResult {
    pub success: bool,
    pub output_path: String,
    pub original_size: u64,
    pub compressed_size: u64,
    pub message: String,
}

#[command]
pub async fn compress_video(
    app: AppHandle,
    request: VideoCompressRequest,
) -> Result<VideoCompressResult, String> {
    let input = Path::new(&request.input_path);
    let output = Path::new(&request.output_path);

    if !input.exists() {
        return Err("输入视频文件不存在".to_string());
    }

    if let Some(parent) = output.parent() {
        let _ = std::fs::create_dir_all(parent);
    }

    let mut args = vec!["-i", input.to_str().ok_or("输入路径非法")?];

    if let Some(resolution) = &request.resolution {
        if resolution != "original" {
            args.push("-vf");
            args.push(&format!("scale={}", resolution));
        }
    }

    if let Some(fps) = request.fps {
        if fps > 0 {
            args.push("-r");
            args.push(&fps.to_string());
        }
    }

    let crf = match request.quality.as_str() {
        "high" => "23",
        "medium" => "28",
        "low" => "34",
        _ => "28",
    };

    args.extend_from_slice(&[
        "-c:v", "libx264",
        "-crf", crf,
        "-c:a", "aac",
        "-b:a", "128k",
        "-y",
        output.to_str().ok_or("输出路径非法")?,
    ]);

    let mut child = tokio::process::Command::new("ffmpeg")
        .args(&args)
        .stderr(Stdio::piped())
        .stdout(Stdio::null())
        .spawn()
        .map_err(|e| format!("启动 FFmpeg 失败：{}", e))?;

    let mut stderr = child.stderr.take()
        .ok_or("无法读取 FFmpeg 输出")?;

    let app_clone = app.clone();
    tokio::spawn(async move {
        let mut reader = tokio::io::BufReader::new(&mut stderr);
        let mut line = String::new();

        loop {
            line.clear();
            match tokio::io::BufRead::read_line(&mut reader, &mut line).await {
                Ok(0) => break,
                Ok(_) => {
                    let line_str = line.trim();
                    if line_str.contains("time=") {
                        if let Some(percent) = parse_progress_percent(line_str) {
                            let _ = app_clone.emit("video_compress_progress", VideoCompressProgress {
                                percent,
                                speed: "".to_string(),
                                eta: "".to_string(),
                            });
                        }
                    }
                }
                Err(e) => {
                    eprintln!("读取 FFmpeg 输出失败：{}", e);
                    break;
                }
            }
        }
    });

    let status = child.wait().await
        .map_err(|e| format!("FFmpeg 执行失败：{}", e))?;

    if !status.success() {
        return Err("视频压缩失败，请检查 FFmpeg 日志".to_string());
    }

    let original_size = std::fs::metadata(input)
        .map_err(|e| e.to_string())?
        .len();

    let compressed_size = std::fs::metadata(output)
        .map_err(|e| e.to_string())?
        .len();

    Ok(VideoCompressResult {
        success: true,
        output_path: request.output_path,
        original_size,
        compressed_size,
        message: "压缩完成".to_string(),
    })
}

fn parse_progress_percent(line: &str) -> Option<f64> {
    let time_str = line.split("time=").nth(1)?;
    let time_str = time_str.split_whitespace().next()?;

    let parts: Vec<&str> = time_str.split(':').collect();
    if parts.len() != 3 {
        return None;
    }

    let hours: f64 = parts[0].parse().ok()?;
    let minutes: f64 = parts[1].parse().ok()?;
    let seconds: f64 = parts[2].split('.').next()?.parse().ok()?;

    let current = hours * 3600.0 + minutes * 60.0 + seconds;

    Some(current / 600.0 * 100.0)
}
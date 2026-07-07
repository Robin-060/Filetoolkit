// 视频处理命令(角色 A · M2 阶段)
//
// 功能:剪切 / 转码 / GPU 检测 / GIF 生成 / 音频提取
// 底层统一调用 ffmpeg 子进程,通过 tokio::spawn_blocking 避免阻塞主线程。

use std::path::PathBuf;
use std::process::Stdio;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::io::BufRead;

use regex::Regex;
use tauri::{AppHandle, Emitter};

use crate::common::dependency;
use crate::common::error::{AppError, AppResult};
use crate::common::types::Progress;

// ============================================================
// 辅助函数
// ============================================================

fn ffmpeg_exe(base: &PathBuf) -> PathBuf {
    base.join(if cfg!(windows) { "ffmpeg.exe" } else { "ffmpeg" })
}

fn ffprobe_exe(base: &PathBuf) -> PathBuf {
    base.join(if cfg!(windows) { "ffprobe.exe" } else { "ffprobe" })
}

fn get_video_duration(input: &str, ffmpeg_path: &PathBuf) -> AppResult<f64> {
    let output = std::process::Command::new(ffprobe_exe(ffmpeg_path))
        .args(["-v", "error", "-show_entries", "format=duration",
               "-of", "default=noprint_wrappers=1:nokey=1", input])
        .output()
        .map_err(|e| AppError::DependencyNotFound(format!("无法运行 ffprobe: {}", e)))?;
    let stdout = String::from_utf8_lossy(&output.stdout);
    stdout.trim().parse::<f64>()
        .map_err(|_| AppError::ProcessingFailed("无法解析视频时长".into()))
}

fn parse_ffmpeg_time(line: &str) -> Option<f64> {
    let re = Regex::new(r"time=(\d+):(\d+):(\d+\.?\d*)").unwrap();
    re.captures(line).map(|caps| {
        let h: f64 = caps[1].parse().unwrap_or(0.0);
        let m: f64 = caps[2].parse().unwrap_or(0.0);
        let s: f64 = caps[3].parse().unwrap_or(0.0);
        h * 3600.0 + m * 60.0 + s
    })
}

fn run_ffmpeg(
    app: &AppHandle,
    ffmpeg_path: &PathBuf,
    args: &[String],
    total_duration_secs: Option<f64>,
    cancel_flag: Arc<AtomicBool>,
) -> AppResult<(bool, String)> {
    let mut child = std::process::Command::new(ffmpeg_exe(ffmpeg_path))
        .args(args)
        .stdout(Stdio::null())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| AppError::ProcessingFailed(format!("无法启动 ffmpeg: {}", e)))?;

    let stderr = child.stderr.take()
        .ok_or_else(|| AppError::ProcessingFailed("无法获取 ffmpeg stderr".into()))?;

    let reader = std::io::BufReader::new(stderr);
    let app_clone = app.clone();
    let mut stderr_text = String::new();
    let flag_inner = cancel_flag.clone();

    let handle = std::thread::spawn(move || {
        for line in reader.lines() {
            if flag_inner.load(Ordering::SeqCst) { break; }
            if let Ok(ref line) = line {
                stderr_text.push_str(line);
                stderr_text.push('\n');
                if let Some(total) = total_duration_secs {
                    if let Some(current) = parse_ffmpeg_time(line) {
                        let pct = ((current / total) * 100.0).min(99.0) as u32;
                        let _ = app_clone.emit("task-progress", Progress {
                            current: pct, total: 100,
                            message: format!("处理中... {:.1}%", pct as f64),
                        });
                    }
                }
            }
        }
        stderr_text
    });

    let _ = child.wait();
    let was_cancelled = cancel_flag.load(Ordering::SeqCst);
    if was_cancelled { let _ = child.kill(); }
    let stderr_final = handle.join().unwrap_or_default();
    Ok((was_cancelled, stderr_final))
}

// ============================================================
// GPU 编码器检测
// ============================================================

#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GpuEncoder {
    pub name: String,
    pub codec: String,
    pub description: String,
}

#[tauri::command]
pub fn detect_gpu_encoders() -> Result<Vec<GpuEncoder>, String> {
    let ffmpeg_path = dependency::require_dependency("ffmpeg").map_err(|e| e.to_string())?;
    let output = std::process::Command::new(ffmpeg_exe(&ffmpeg_path))
        .args(["-encoders"])
        .output()
        .map_err(|e| format!("无法运行 ffmpeg: {}", e))?;
    let stdout = String::from_utf8_lossy(&output.stdout);

    let patterns: &[(&str, &str)] = &[
        ("h264_nvenc", "NVIDIA NVENC H.264"), ("hevc_nvenc", "NVIDIA NVENC H.265/HEVC"),
        ("h264_qsv", "Intel QSV H.264"), ("hevc_qsv", "Intel QSV H.265/HEVC"),
        ("h264_videotoolbox", "Apple VideoToolbox H.264"), ("hevc_videotoolbox", "Apple VideoToolbox H.265/HEVC"),
        ("h264_amf", "AMD AMF H.264"), ("hevc_amf", "AMD AMF H.265/HEVC"),
    ];

    let mut encoders = Vec::new();
    for (codec, desc) in patterns {
        if stdout.contains(codec) {
            let vendor = if codec.contains("nvenc") { "NVIDIA" }
                else if codec.contains("qsv") { "Intel" }
                else if codec.contains("videotoolbox") { "Apple" }
                else if codec.contains("amf") { "AMD" }
                else { "Unknown" };
            encoders.push(GpuEncoder { name: vendor.into(), codec: codec.to_string(), description: desc.to_string() });
        }
    }
    Ok(encoders)
}

// ============================================================
// 视频剪切
// ============================================================

#[tauri::command]
pub async fn cut_video(
    app: AppHandle, input: String, start: String, end: String,
    output: String, mode: Option<String>,
) -> Result<String, String> {
    let ffmpeg_path = dependency::require_dependency("ffmpeg").map_err(|e| e.to_string())?;
    let is_fast = mode.as_deref() != Some("accurate");
    let cancel_flag = Arc::new(AtomicBool::new(false));

    tokio::task::spawn_blocking(move || {
        let mut args: Vec<String> = vec![
            "-y".into(), "-i".into(), input,
            "-ss".into(), start, "-to".into(), end,
        ];
        if is_fast { args.push("-c".into()); args.push("copy".into()); }
        let out = output.clone();
        args.push(output);

        let (was_cancelled, _) = run_ffmpeg(&app, &ffmpeg_path, &args, None, cancel_flag.clone())
            .map_err(|e| e.to_string())?;
        if was_cancelled { Err("任务已取消".into()) }
        else { Ok(format!("剪切完成: {}", out)) }
    }).await.map_err(|e| format!("任务失败: {}", e))?
}

// ============================================================
// 视频转码
// ============================================================

#[tauri::command]
pub async fn transcode_video(
    app: AppHandle, input: String, _output_format: String,
    video_codec: String, crf: Option<u8>, encoder: Option<String>, output: String,
) -> Result<String, String> {
    let ffmpeg_path = dependency::require_dependency("ffmpeg").map_err(|e| e.to_string())?;
    let actual_encoder = encoder.unwrap_or_else(|| match video_codec.as_str() {
        "h264" => "libx264".into(), "h265" => "libx265".into(),
        "vp9" => "libvpx-vp9".into(), "av1" => "libaom-av1".into(),
        _ => "libx264".into(),
    });
    let crf_val = crf.unwrap_or(23);
    let cancel_flag = Arc::new(AtomicBool::new(false));
    let input_for_dur = input.clone();
    let ffmpeg_for_dur = ffmpeg_path.clone();

    tokio::task::spawn_blocking(move || {
        let duration = get_video_duration(&input_for_dur, &ffmpeg_for_dur).ok();
        let args: Vec<String> = vec![
            "-y".into(), "-i".into(), input, "-c:v".into(), actual_encoder,
            "-crf".into(), crf_val.to_string(), "-preset".into(), "medium".into(),
            "-c:a".into(), "aac".into(), "-b:a".into(), "128k".into(), output,
        ];
        let (was_cancelled, _) = run_ffmpeg(&app, &ffmpeg_path, &args, duration, cancel_flag.clone())
            .map_err(|e| e.to_string())?;
        if was_cancelled { Err("任务已取消".into()) }
        else { Ok(String::from("转码完成")) }
    }).await.map_err(|e| format!("任务失败: {}", e))?
}

// ============================================================
// 视频 → GIF
// ============================================================

#[tauri::command]
pub async fn video_to_gif(
    app: AppHandle, input: String, start: String, duration: f64,
    fps: Option<u32>, width: Option<u32>, output: String,
) -> Result<String, String> {
    let ffmpeg_path = dependency::require_dependency("ffmpeg").map_err(|e| e.to_string())?;
    let fps_val = fps.unwrap_or(10);
    let width_val = width.unwrap_or(480);
    let cancel_flag = Arc::new(AtomicBool::new(false));

    tokio::task::spawn_blocking(move || {
        let filter = format!(
            "fps={},scale={}:-1:flags=lanczos,split[s0][s1];[s0]palettegen[p];[s1][p]paletteuse",
            fps_val, width_val
        );
        let args: Vec<String> = vec![
            "-y".into(), "-ss".into(), start, "-t".into(), duration.to_string(),
            "-i".into(), input, "-filter_complex".into(), filter, output,
        ];
        let (_, _) = run_ffmpeg(&app, &ffmpeg_path, &args, Some(duration), cancel_flag.clone())
            .map_err(|e| e.to_string())?;
        Ok(String::from("GIF 生成完成"))
    }).await.map_err(|e| format!("任务失败: {}", e))?
}

// ============================================================
// 提取音频
// ============================================================

#[tauri::command]
pub async fn extract_audio(
    app: AppHandle, input: String, format: String,
    bitrate: Option<String>, output: String,
) -> Result<String, String> {
    let ffmpeg_path = dependency::require_dependency("ffmpeg").map_err(|e| e.to_string())?;
    let br = bitrate.unwrap_or_else(|| "192k".into());
    let codec = match format.as_str() {
        "mp3" => "libmp3lame", "aac" => "aac", "flac" => "flac",
        "ogg" => "libvorbis", _ => "pcm_s16le", // wav default
    }.to_string();
    let cancel_flag = Arc::new(AtomicBool::new(false));

    tokio::task::spawn_blocking(move || {
        let args: Vec<String> = vec![
            "-y".into(), "-i".into(), input, "-vn".into(),
            "-c:a".into(), codec, "-b:a".into(), br, output,
        ];
        let (_, _) = run_ffmpeg(&app, &ffmpeg_path, &args, None, cancel_flag.clone())
            .map_err(|e| e.to_string())?;
        Ok(String::from("音频提取完成"))
    }).await.map_err(|e| format!("任务失败: {}", e))?
}

// ============================================================
// 测试
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_ffmpeg_time_valid() {
        let r = parse_ffmpeg_time("frame=100 fps=24 time=00:01:30.50 bitrate=1000kbits/s");
        assert!(r.is_some());
        assert!((r.unwrap() - 90.5).abs() < 0.01);
    }

    #[test]
    fn test_parse_ffmpeg_time_no_match() {
        assert!(parse_ffmpeg_time("frame=100 fps=24 bitrate=1000kbits/s").is_none());
    }

    #[test]
    fn test_parse_ffmpeg_time_hours() {
        assert_eq!(parse_ffmpeg_time("time=02:00:00.00"), Some(7200.0));
    }
}

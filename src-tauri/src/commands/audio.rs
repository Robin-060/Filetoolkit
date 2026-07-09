use std::process::Command;
use std::env;
use std::fs;

#[tauri::command]
pub async fn convert_audio(
    input: String,
    format: String,
    bitrate: String,
    output: String,
) -> Result<(), String> {
    let codec = match format.as_str() {
        "mp3" => "libmp3lame",
        "aac" => "aac",
        "flac" => "flac",
        "wav" => "pcm_s16le",
        "ogg" => "libvorbis",
        _ => return Err("仅支持 mp3/aac/flac/wav/ogg 格式".to_string()),
    };

    let exec = Command::new("ffmpeg")
        .args([
            "-y",
            "-i",
            &input,
            "-c:a",
            codec,
            "-b:a",
            &bitrate,
            &output,
        ])
        .status();

    match exec {
        Ok(status) if status.success() => Ok(()),
        Ok(_) => Err("音频转换失败，检查文件路径".to_string()),
        Err(e) => Err(format!("FFmpeg 运行失败: {}", e)),
    }
}

#[tauri::command]
pub async fn cut_audio(
    input: String,
    start: String,
    end: String,
    output: String,
) -> Result<(), String> {
    let exec = Command::new("ffmpeg")
        .args([
            "-y",
            "-ss",
            &start,
            "-to",
            &end,
            "-i",
            &input,
            "-c",
            "copy",
            &output,
        ])
        .status();

    match exec {
        Ok(status) if status.success() => Ok(()),
        Ok(_) => Err("音频剪切失败，检查时间格式".to_string()),
        Err(e) => Err(format!("FFmpeg 运行失败: {}", e)),
    }
}

#[tauri::command]
pub async fn merge_audio(files: Vec<String>, output: String) -> Result<(), String> {
    if files.is_empty() {
        return Err("未选择任何音频文件".to_string());
    }

    let mut list_text = String::new();
    for path in files {
        list_text.push_str(&format!("file '{}'\n", path));
    }

    let temp_path = env::temp_dir().join("audio_merge_list.txt");
    fs::write(&temp_path, list_text).map_err(|e| e.to_string())?;

    let exec = Command::new("ffmpeg")
        .args([
            "-y",
            "-f",
            "concat",
            "-safe",
            "0",
            "-i",
            temp_path.to_str().unwrap(),
            "-c",
            "copy",
            &output,
        ])
        .status();

    let _ = fs::remove_file(temp_path);

    match exec {
        Ok(status) if status.success() => Ok(()),
        Ok(_) => Err("音频合并失败".to_string()),
        Err(e) => Err(format!("FFmpeg 运行失败: {}", e)),
    }
}

#[tauri::command]
pub async fn normalize_audio(
    input: String,
    target_lufs: f64,
    output: String,
) -> Result<(), String> {
    let i_val = target_lufs.to_string();
    let tp_val = (target_lufs + 14.5).to_string();
    let filter = format!("loudnorm=I={}:TP={}:LRA=11", i_val, tp_val);

    let exec = Command::new("ffmpeg")
        .args([
            "-y",
            "-i",
            &input,
            "-af",
            &filter,
            &output,
        ])
        .status();

    match exec {
        Ok(status) if status.success() => Ok(()),
        Ok(_) => Err("音量标准化处理失败".to_string()),
        Err(e) => Err(format!("FFmpeg 运行失败: {}", e)),
    }
}
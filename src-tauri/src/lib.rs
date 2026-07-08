// 应用库入口:声明各功能模块,并组装 Tauri 应用。
// 模块职责详见 docs/design/m0-skeleton.md §3.

mod common;
mod commands;
mod pipeline;
mod worker;

pub use commands::rename::{preview_rename, execute_rename};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            commands::greet,
            worker::cancel_batch,
            preview_rename,
            execute_rename,
            commands::video::detect_gpu_encoders,
            commands::video::cut_video,
            commands::video::transcode_video,
            commands::video::video_to_gif,
            commands::video::extract_audio,
            common::dependency::check_dependency,
            common::dependency::check_all_dependencies,
            common::dependency::clear_dependency_cache,
            worker::cancel_batch
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
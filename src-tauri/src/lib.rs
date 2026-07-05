// 应用库入口:声明各功能模块,并组装 Tauri 应用。
// 模块职责详见 docs/design/m0-skeleton.md §3。

mod common;
mod commands;
mod pipeline;
mod worker;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![commands::greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

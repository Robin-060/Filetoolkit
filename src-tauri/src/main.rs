use tauri::Builder;

mod commands;
use commands::*;

fn main() {
    Builder::default()
        // 正确宏写法：![] 包裹所有命令，彻底解决宏报错
        .invoke_handler(tauri::generate_handler![
            rename_file,
            scan_duplicate_files,
            delete_duplicate_file,
            get_video_files
        ])
        .run(tauri::generate_context!())
        .expect("应用启动失败");
}
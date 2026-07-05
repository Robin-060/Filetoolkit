// commands 模块:暴露给前端调用的 Tauri 命令。
// 每个功能独立成子模块,职责单一。
// 详见 docs/design/m0-skeleton.md §3 与 docs/PRD.md §3.1。

pub mod image; // 图片处理(M1):压缩/转换/裁剪
pub mod pdf; // PDF 处理(M1):合并/拆分/压缩
pub mod video; // 视频处理(M2):剪切/转码/压缩
pub mod rename; // 批量重命名(M1)
pub mod dedup; // 重复文件查重(M1)

/// 调通命令:M0 阶段用于验证前后端通信闭环。
/// 后续 MVP 功能上线后保留,可用于健康检查。
#[tauri::command]
pub fn greet(name: &str) -> String {
    format!("来自 Rust 后端的问候:你好,{}!FileToolkit 已就绪。", name)
}

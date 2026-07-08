// 共享类型、错误与工具函数。
pub mod dependency; // 可选外部依赖探测(ffmpeg/LibreOffice/Tesseract)
pub mod error;     // 统一错误类型(thiserror)
pub mod types;     // 共享结构体(Task / Progress 等)

#[allow(unused)]
pub use crate::commands::dedup::{DuplicateGroup, FileInfo};
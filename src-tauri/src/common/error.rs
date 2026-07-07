// 统一错误类型 —— 所有后端命令的返回值都使用这个枚举。
// 通过 serde::Serialize 可将错误信息透传给前端。
// 此文件是项目后端基础,成员 B/C 的所有命令都依赖它。

use serde::Serialize;
use thiserror::Error;

/// 应用全局错误类型。
/// 每个变体实现 `Serialize`,前端可以接收并展示中文错误消息。
#[derive(Error, Debug, Serialize)]
pub enum AppError {
    #[error("文件不存在: {0}")]
    FileNotFound(String),

    #[error("IO 错误: {0}")]
    Io(String),

    #[error("不支持的格式: {0}")]
    UnsupportedFormat(String),

    #[error("任务已取消")]
    TaskCancelled,

    #[error("处理失败: {0}")]
    ProcessingFailed(String),

    #[error("未找到依赖: {0}。请安装后重试。")]
    DependencyNotFound(String),

    #[error("参数无效: {0}")]
    InvalidParameter(String),

    #[error("权限不足: {0}")]
    PermissionDenied(String),
}

// —— 标准转换实现,让 ? 操作符能自动转换常见错误 ——

impl From<std::io::Error> for AppError {
    fn from(e: std::io::Error) -> Self {
        AppError::Io(e.to_string())
    }
}

impl From<serde_json::Error> for AppError {
    fn from(e: serde_json::Error) -> Self {
        AppError::ProcessingFailed(format!("JSON 序列化失败: {}", e))
    }
}

/// AppError 的便捷类型别名:Result<T, AppError>
pub type AppResult<T> = Result<T, AppError>;

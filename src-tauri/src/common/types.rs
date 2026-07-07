// 共享数据结构 —— 在 commands、pipeline、worker 之间流转的通用类型。
// 成员 B/C 实现命令时,使用这些类型与 worker 框架交互。

use serde::{Deserialize, Serialize};

/// 任务状态枚举。
/// 前端根据此状态展示不同 UI(进度条/成功/失败/取消)。
/// 序列化为 `{"type": "running"}` 或 `{"type": "failed", "message": "..."}`。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", content = "message", rename_all = "camelCase")]
pub enum TaskStatus {
    /// 等待执行
    Pending,
    /// 正在处理
    Running,
    /// 已完成
    Completed,
    /// 处理失败(附带错误消息)
    Failed(String),
    /// 用户取消
    Cancelled,
}

/// 单个任务的定义。
/// worker 拿到 Vec<Task> 后按顺序或并行执行。
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Task {
    /// 任务唯一标识(UUID)
    pub id: String,
    /// 输入文件路径
    pub input_path: String,
    /// 期望的输出文件路径
    pub output_path: String,
    /// 当前状态
    pub status: TaskStatus,
    /// 附加参数(JSON),各命令自行解析
    #[serde(default)]
    pub params: serde_json::Value,
}

/// 任务执行结果,回传给前端。
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskResult {
    /// 对应 task id
    pub task_id: String,
    /// 输入路径
    pub input_path: String,
    /// 最终输出路径
    pub output_path: String,
    /// 原文件大小(字节),可选(如图片压缩场景)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_size: Option<u64>,
    /// 新文件大小(字节),可选
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_size: Option<u64>,
    /// 执行状态
    pub status: TaskStatus,
}

/// 进度上报结构,通过 Tauri 事件 `task-progress` 推给前端。
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Progress {
    /// 已完成数量
    pub current: u32,
    /// 总数量
    pub total: u32,
    /// 附加信息(如当前处理的文件名)
    #[serde(default)]
    pub message: String,
}

// 任务队列与进度上报框架 —— 批量操作的统一执行引擎。
// 成员 B/C 在他们的命令中创建 BatchRunner,传入任务列表和处理闭包即可获得:
//   - 多核并行(rayon)
//   - 实时进度事件(task-progress)
//   - 单个失败不中断整批
//   - 用户可取消

use std::sync::{Arc, Mutex, atomic::{AtomicBool, Ordering}};

use rayon::ThreadPoolBuilder;
use tauri::{AppHandle, Emitter};

use crate::common::error::{AppError, AppResult};
use crate::common::types::{Progress, Task, TaskResult, TaskStatus};

// ============================================================
// 全局取消标志
// ============================================================

/// 当前活跃的 BatchRunner 的取消标志。
/// cancel_batch 命令通过此全局变量通知正在运行的任务停止。
static CURRENT_CANCEL_FLAG: Mutex<Option<Arc<AtomicBool>>> = Mutex::new(None);

fn set_cancel_flag(flag: Arc<AtomicBool>) {
    let mut guard = CURRENT_CANCEL_FLAG.lock().unwrap();
    *guard = Some(flag);
}

fn clear_cancel_flag() {
    let mut guard = CURRENT_CANCEL_FLAG.lock().unwrap();
    *guard = None;
}

fn get_cancel_flag() -> Option<Arc<AtomicBool>> {
    CURRENT_CANCEL_FLAG.lock().unwrap().clone()
}

/// Tauri 命令:取消当前正在执行的批量任务。
/// 前端点取消按钮时调用此命令。
#[tauri::command]
pub fn cancel_batch() -> Result<(), String> {
    if let Some(flag) = get_cancel_flag() {
        flag.store(true, Ordering::SeqCst);
        Ok(())
    } else {
        Err("没有正在执行的任务".into())
    }
}

// ============================================================
// BatchRunner
// ============================================================

/// 批量任务执行器。
///
/// # 用法(成员 B/C 在命令中这样用):
///
/// ```ignore
/// let runner = BatchRunner::new(app_handle);
/// let results = runner.run(tasks, |task| {
///     // 处理单个文件,返回处理后的内容或错误
///     do_something(&task.input_path)
/// })?;
/// ```
pub struct BatchRunner {
    app: AppHandle,
    cancel_flag: Arc<AtomicBool>,
}

impl BatchRunner {
    /// 创建一个新的批量执行器,自动注册到全局取消标志。
    pub fn new(app: AppHandle) -> Self {
        let cancel_flag = Arc::new(AtomicBool::new(false));
        set_cancel_flag(cancel_flag.clone());
        Self { app, cancel_flag }
    }

    /// 手动触发取消(内部也通过 cancel_batch 命令调用)。
    pub fn cancel(&self) {
        self.cancel_flag.store(true, Ordering::SeqCst);
    }

    /// 检查是否已被取消。处理闭包中应定期调用此方法。
    pub fn is_cancelled(&self) -> bool {
        self.cancel_flag.load(Ordering::SeqCst)
    }

    /// 核心执行方法:并行处理一组任务。
    ///
    /// - `tasks`:要处理的任务列表。
    /// - `process_one`:处理单个 task 的函数。
    ///   返回 `Ok(output_message)` 表示成功,
    ///   返回 `Err(AppError)` 表示失败(不中断其余 task)。
    pub fn run<F>(&self, tasks: Vec<Task>, process_one: F) -> AppResult<Vec<TaskResult>>
    where
        F: Fn(&Task, &BatchRunner) -> AppResult<String> + Send + Sync,
    {
        let total = tasks.len() as u32;
        let app = self.app.clone();

        // 发送初始进度
        let _ = app.emit(
            "task-progress",
            Progress {
                current: 0,
                total,
                message: "开始处理...".into(),
            },
        );

        // 构建线程池(使用所有可用核心)
        let pool = ThreadPoolBuilder::new()
            .num_threads(num_cpus::get())
            .build()
            .map_err(|e| AppError::ProcessingFailed(format!("创建线程池失败: {}", e)))?;

        // 并行处理,收集结果
        let results: Vec<TaskResult> = pool.install(|| {
            use rayon::prelude::*;

            tasks
                .par_iter()
                .map(|task| {
                    // 检查取消标志
                    if self.is_cancelled() {
                        return TaskResult {
                            task_id: task.id.clone(),
                            input_path: task.input_path.clone(),
                            output_path: task.output_path.clone(),
                            original_size: None,
                            new_size: None,
                            status: TaskStatus::Cancelled,
                        };
                    }

                    match process_one(task, self) {
                        Ok(_msg) => TaskResult {
                            task_id: task.id.clone(),
                            input_path: task.input_path.clone(),
                            output_path: task.output_path.clone(),
                            original_size: get_file_size(&task.input_path),
                            new_size: get_file_size(&task.output_path),
                            status: TaskStatus::Completed,
                        },
                        Err(e) => TaskResult {
                            task_id: task.id.clone(),
                            input_path: task.input_path.clone(),
                            output_path: task.output_path.clone(),
                            original_size: None,
                            new_size: None,
                            status: TaskStatus::Failed(e.to_string()),
                        },
                    }
                })
                .collect()
        });

        // 发送完成进度
        let failed_count = results.iter().filter(|r| matches!(r.status, TaskStatus::Failed(_))).count();
        let cancelled_count = results.iter().filter(|r| r.status == TaskStatus::Cancelled).count();
        let _ = app.emit(
            "task-progress",
            Progress {
                current: total,
                total,
                message: format!(
                    "完成:{} 成功,{} 失败,{} 取消",
                    total as usize - failed_count - cancelled_count,
                    failed_count,
                    cancelled_count
                ),
            },
        );

        Ok(results)
    }
}

impl Drop for BatchRunner {
    fn drop(&mut self) {
        clear_cancel_flag();
    }
}

// ============================================================
// 辅助
// ============================================================

fn get_file_size(path: &str) -> Option<u64> {
    std::fs::metadata(path).ok().map(|m| m.len())
}

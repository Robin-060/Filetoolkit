# 成员 A 任务卡 — 后端核心架构

> **你的角色**:Rust 后端核心。你写的东西是所有其他后端功能的地基和引擎,必须最先动工。
> **代号**:Core

---

## 一、开始前

参照 [`docs/CONTRIBUTING.md`](../CONTRIBUTING.md) 第一章:
```bash
git clone https://github.com/Robin-060/file-toolkit.git
cd file-toolkit
pnpm install && pnpm tauri dev   # 弹窗即成功
```

---

## 二、你要做的全部步骤(按顺序)

| 序号 | 步骤 | 里程碑 | 依赖谁 | 可并行 |
|:--:|------|:--:|------|:--:|
| A-1 | 统一错误处理与共享类型 | M1 | 无(你最先动) | — |
| A-2 | 任务队列与进度上报框架 | M1 | A-1 | — |
| A-3 | ffmpeg 可选依赖管理 | M2 | M1 完成 | — |
| A-4 | 视频剪切/转码 | M2 | A-3 | — |
| A-5 | 硬件加速 + 压缩 + GIF | M2 | A-4 | — |
| A-6 | 流水线数据模型与执行引擎 | M3 | M1 完成 | — |
| A-7 | 节点类型注册表 | M3 | A-6 | E 可并行做前端 |
| A-8 | Dry-run 预览与执行控制 | M3 | A-6 | — |

---

## 三、每步详解

### A-1 — 统一错误处理与共享类型(Step 1)

**做什么**:

1. 在 `src-tauri/Cargo.toml` 加依赖:
   ```bash
   cd src-tauri && cargo add thiserror serde
   ```

2. 编写 `src-tauri/src/common/error.rs`:
   ```rust
   use serde::Serialize;
   use thiserror::Error;

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
   }

   impl From<std::io::Error> for AppError {
       fn from(e: std::io::Error) -> Self {
           AppError::Io(e.to_string())
       }
   }
   ```

3. 编写 `src-tauri/src/common/types.rs`:
   ```rust
   use serde::{Deserialize, Serialize};

   #[derive(Debug, Clone, Serialize, Deserialize)]
   pub enum TaskStatus {
       Pending,
       Running,
       Completed,
       Failed(String),
       Cancelled,
   }

   #[derive(Debug, Clone, Serialize, Deserialize)]
   pub struct Task {
       pub id: String,
       pub input_path: String,
       pub output_path: String,
       pub status: TaskStatus,
   }

   #[derive(Debug, Clone, Serialize, Deserialize)]
   pub struct Progress {
       pub current: u32,
       pub total: u32,
       pub message: String,
   }
   ```

4. 确保 `src-tauri/src/common/mod.rs` 声明了:
   ```rust
   pub mod error;
   pub mod types;
   ```

**验收**:`cargo check --manifest-path src-tauri/Cargo.toml` 通过。类型可序列化。

---

### A-2 — 任务队列与进度上报框架(Step 2)

**做什么**:

1. 加依赖:`cd src-tauri && cargo add rayon uuid`

2. 编写 `src-tauri/src/worker/mod.rs`——核心是一个并行任务执行器:
   - 接收 `Vec<Task>` + 一个处理函数(闭包)
   - 用 `rayon::ThreadPool` 并行执行,每处理完一个 task,通过 `app_handle.emit("task-progress", progress)` 发事件
   - 用 `Arc<AtomicBool>` 做取消标志:前端点取消后,正在跑的 task 在下一个检查点退出
   - 单个 task 失败 → 将其 `status` 标记为 `Failed`,**不中断其余**

3. 架构参考:
   ```rust
   use rayon::ThreadPoolBuilder;
   use std::sync::{Arc, atomic::{AtomicBool, Ordering}};
   use tauri::{AppHandle, Emitter};
   use crate::common::types::{Task, Progress, TaskStatus};

   pub struct BatchRunner {
       cancel_flag: Arc<AtomicBool>,
   }

   impl BatchRunner {
       pub fn new() -> Self { ... }
       pub fn cancel(&self) { self.cancel_flag.store(true, Ordering::SeqCst); }

       pub fn run<F>(&self, app: AppHandle, tasks: Vec<Task>, process_one: F)
       where F: Fn(&Task) -> Result<String, crate::common::error::AppError> + Send + Sync
       { ... }
   }
   ```

**验收**:给 100 个模拟 task 跑,前端能收到进度事件;其中某个故意失败,其余 99 个继续;调用 `cancel()` 后在 1-2 秒内停下。

> ⚠️ 前端还没写好事件监听时,你可以先用 `cargo test` 验证逻辑,事件监听由成员 D 在 Step 3 中实现。

---

### A-3 — ffmpeg 可选依赖管理(Step 11)

**做什么**:

1. 编写 `src-tauri/src/common/dependency.rs`:
   - `pub fn detect_ffmpeg() -> Option<PathBuf>`:先在 PATH 里搜,再搜常见安装路径
   - `pub struct DependencyStatus { found: bool, path: Option<String>, guidance: String }`
   - 首次探测结果缓存到 `dirs::cache_dir()` 下的 JSON 文件,下次秒开
   - 暴露一个 Tauri 命令 `check_ffmpeg` 给前端,返回 `DependencyStatus`

2. 下载脚本(放 `scripts/` 目录):
   - Windows:`download-ffmpeg.ps1`——从 gyan.dev 下载 essentials build
   - 脚本逻辑:下载 → 解压到 `%LOCALAPPDATA%/file-toolkit/ffmpeg/` → 返回路径

3. 前端配合:缺失时弹出引导框("一键下载"按钮调脚本/"手动指定路径"按钮选文件夹)

**依赖**:`cd src-tauri && cargo add dirs`

**验收**:有 ffmpeg 时前端显示 ✅;无时弹出绿色引导;缓存后重启不重复探测。

---

### A-4 — 视频剪切/转码(Step 12)

**做什么**:

1. 编写 `src-tauri/src/commands/video.rs`:

   **剪切命令**:
   ```rust
   #[tauri::command]
   async fn cut_video(app: AppHandle, input: String, start: String, end: String, output: String) -> Result<String, String>
   ```
   - 调 ffmpeg:`ffmpeg -i input -ss start -to end -c copy output`(无损,秒级)
   - 解析 ffmpeg stderr 中的 `time=` 行,提取时间百分比 → emit 进度

   **转码命令**:
   ```rust
   #[tauri::command]
   async fn transcode_video(app: AppHandle, input: String, format: String, codec: String, output: String) -> Result<String, String>
   ```
   - `ffmpeg -i input -c:v libx264/libx265 -f format output`
   - 同样解析 stderr 取进度

2. **进度解析参考**(Rust 中调 ffmpeg 子进程):
   ```rust
   use std::process::Stdio;
   use tokio::process::Command;
   // 用 Regex 从 stderr 中提取 "time=00:01:23.45" → 计算百分比
   ```

3. 在 `lib.rs` 注册这两个命令。

**前端**:成员 E 负责 `pages/VideoPage.vue`(时间轴选择、格式/编码下拉、进度条),他会在你写好命令后对接。

**验收**:精确裁剪,无损秒级完成;转码输出正确。

---

### A-5 — 硬件加速 + 压缩 + GIF(Step 13)

**做什么**:

1. **硬件加速检测**:
   ```rust
   fn detect_gpu() -> Vec<GpuEncoder> {
       // Windows: 检查注册表或运行 nvidia-smi
       // 返回可用编码器列表: NVENC / QSV / VideoToolbox(macOS)
   }
   ```
   - 在转码命令中添加 `--hwaccel` 参数
   - N 卡 → NVENC(`h264_nvenc`);Intel → QSV(`h264_qsv`);Mac → videotoolbox

2. **GIF 生成**:
   ```rust
   #[tauri::command]
   async fn video_to_gif(input: String, start: String, duration: f64, fps: u32, width: u32, output: String) -> ...
   ```
   - `ffmpeg -i input -ss start -t duration -vf "fps=10,scale=320:-1" output.gif`

3. **音频提取**:
   ```rust
   #[tauri::command]
   async fn extract_audio(input: String, format: String, output: String) -> ...
   ```

**验收**:N 卡自动用 NVENC 加速;GIF 帧率/尺寸可调;音频提取正确。

---

### A-6 — 流水线数据模型与执行引擎(Step 17)

**做什么**:

⚠️ 这步要等 M1 四个功能完成后再做(否则没有节点可串联)。

1. 编写 `src-tauri/src/pipeline/model.rs`:
   ```rust
   pub struct Pipeline {
       pub nodes: Vec<Node>,
       pub edges: Vec<Edge>,
   }
   pub struct Node {
       pub id: String,
       pub node_type: String,    // "image_compress" | "pdf_merge" | "rename" | ...
       pub params: serde_json::Value,
   }
   pub struct Edge {
       pub from: String,          // 上游节点 id
       pub to: String,            // 下游节点 id
   }
   ```

2. 编写 `src-tauri/src/pipeline/executor.rs`:
   - 输入 `Pipeline` → DAG 拓扑排序(拓扑序相同层可并行)
   - 按层执行,每层内用 `rayon` 并行
   - 变量传递:上游节点的输出(如提取的 EXIF 日期)作为下游的输入
   - 失败策略:Stop(当前节点失败则停) / Skip(跳过失败的继续)

**验收**:JSON 描述的流水线能拓扑排序;串行+并行混合执行正确。

---

### A-7 — 节点类型注册表(Step 18)

**做什么**:

编写 `src-tauri/src/pipeline/registry.rs`:

```rust
use std::collections::HashMap;

pub struct NodeType {
    pub id: String,
    pub name: String,               // 显示名
    pub description: String,
    pub input_schema: serde_json::Value,   // 输入参数 JSON Schema
    pub output_schema: serde_json::Value,  // 输出格式
}

pub fn get_registry() -> HashMap<String, NodeType> {
    // 注册所有原子功能:
    //   image_compress, pdf_merge, pdf_split, pdf_compress,
    //   rename, dedup, video_cut, video_transcode, audio_convert, ...
}
```

**验收**:每个 M1/M2 功能都可用作流水线节点。

---

### A-8 — Dry-run 预览与执行控制(Step 21)

**做什么**:

1. 编写 `src-tauri/src/pipeline/preview.rs`:
   ```rust
   pub fn dry_run(pipeline: &Pipeline, input_files: &[String]) -> Vec<FileTransform> {
       // 对每个输入文件,模拟走过 pipeline 的每个节点
       // 返回完整的变换链(原文件名 → 经过节点1 → 经过节点2 → 最终文件名)
   }
   ```

2. 执行控制:前端[执行]按钮 → 后端开始跑 → 可[暂停]/[继续]/[取消]

**前端配合**:成员 D 负责可视化编辑器和执行控制 UI。

**验收**:dry-run 列出完整计划;执行中可暂停,恢复后状态一致。

---

## 四、协作点

| 你需要给谁什么东西 | 谁需要 | 时机 |
|---|---|---|
| `common/error.rs` + `common/types.rs` | B、C(所有后端功能都用) | A-1 完成后立即 |
| `worker/mod.rs` 的任务执行接口 | B、C(功能中调用 worker) | A-2 完成后立即 |
| `common/dependency.rs` 的 ffmpeg 探测 | E(前端引导 UI) | A-3 完成后 |
| `commands/video.rs` 的命令签名 | E(前端 VideoPage) | A-4 完成前先约定接口 |
| `pipeline/registry.rs` 的节点类型定义 | D(可视化编辑器) | A-7 完成前先约定 schema |

---

## 五、验收总清单

- [ ] A-1:`cargo check` 通过,AppError 可序列化
- [ ] A-2:100 task 并行,失败隔离,可取消,进度 emit
- [ ] A-3:ffmpeg 探测+缓存+引导下载
- [ ] A-4:视频无损剪切秒级,转码进度百分比
- [ ] A-5:NVENC 自动启用,GIF 可调参数,音频可提取
- [ ] A-6:DAG 拓扑排序,变量传递,失败策略
- [ ] A-7:全部功能注册为节点
- [ ] A-8:dry-run 预览,暂停/继续/取消

---

> 📞 遇到问题找项目负责人。A-1 和 A-2 是你最关键的产出,后面所有人的代码都依赖它们,优先做好。

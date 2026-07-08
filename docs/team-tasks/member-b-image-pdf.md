# 成员 B 任务卡 — 图片与PDF功能

> **你的角色**:图片处理 + PDF 处理的全栈开发。从 Rust 后端写到 Vue 前端,独立交付两个完整功能。
> **代号**:ImagePDF

---

## 一、开始前

参照 [`docs/CONTRIBUTING.md`](../CONTRIBUTING.md) 第一章:
```bash
git clone https://github.com/Robin-060/file-toolkit.git
cd file-toolkit
pnpm install && pnpm tauri dev
```

---

## 二、你要做的全部步骤(按顺序)

| 序号 | 步骤 | 里程碑 | 依赖谁 | 可并行 |
|:--:|------|:--:|------|:--:|
| B-1 | 图片批量压缩/转换(全栈) | M1 | A-1, A-2, D-1 | C 同时做 C-1 |
| B-2 | PDF 合并/拆分/压缩(后端) | M1 | A-1, A-2 | E 同时做 PDF 前端 |
| B-3 | Office 文档转换 | M4 | A-3 | 可与 C-4/C-5 并行 |
| B-4 | 批量解压/压缩 | M4 | 无 | 可与 B-3 并行 |

---

## 三、每步详解

### B-1 — 图片批量压缩/转换(Step 4,全栈)

**后端 `commands/image.rs`**:

1. 加依赖:`cd src-tauri && cargo add image`

2. 实现命令:
   ```rust
   #[tauri::command]
   async fn compress_images(
       app: AppHandle,
       files: Vec<String>,
       quality: u8,          // 0-100
       format: String,       // "jpg" | "png" | "webp"
       max_width: Option<u32>,
       max_height: Option<u32>,
       output_dir: String,
   ) -> Result<Vec<TaskResult>, String>
   ```

3. 处理流程:
   ```
   遍历 files:
     image::open(file) → 如需 resize → 按 quality/format 编码 → 写入 output_dir
     → emit("task-progress", {current, total})
     → 返回 {原大小, 新大小, 状态}
   ```

4. 调用成员 A 的 worker 框架(接口:A-2 完成后他会给你):
   ```rust
   use crate::worker::BatchRunner;
   // 用 BatchRunner 跑你的处理闭包
   ```

5. 在 `lib.rs` 的 `generate_handler![]` 中注册 `compress_images`。

**前端 `pages/ImagePage.vue`**:

6. UI 布局:
   - **拖拽区**:成员 D 会给你 `FileDropZone.vue` 组件(D-2 产出)。在 D 完成前,先用 `<input type="file" multiple>` 临时替代
   - **参数面板**:输出格式(JPG/PNG/WebP 下拉)、质量滑块(0-100)、宽度/高度(可选,输数字)
   - **输出目录**:文件夹选择按钮
   - **执行按钮** + **进度条**(用 D 的 `TaskProgress.vue`)
   - **结果列表**:每个文件显示:原文件名 → 原大小 → 新大小 → 压缩率

7. 调用后端(参考 `App.vue` 中的 `greet` 模式):
   ```typescript
   import { invoke } from "@tauri-apps/api/core";
   const results = await invoke("compress_images", {
     files: [...], quality: 80, format: "webp",
     maxWidth: 1920, maxHeight: null, outputDir: "..."
   });
   ```

**验收**:
- [ ] 拖入 50 张图,JPG 转 WebP,质量 80%,全部成功
- [ ] 处理后能看到 原大小 vs 新大小
- [ ] 坏文件被跳过不影响其余
- [ ] 进度条实时更新
- [ ] 处理中可取消

---

### B-2 — PDF 合并/拆分/压缩(后端)(Step 5)

> ⚠️ 这步你只写**后端**,前端由成员 E 负责。

**后端 `commands/pdf.rs`**:

1. 加依赖:`cd src-tauri && cargo add lopdf`

2. 实现三个命令:

   **合并**:
   ```rust
   #[tauri::command]
   async fn merge_pdfs(files: Vec<String>, output_path: String) -> Result<String, String>
   ```
   - 用 `lopdf::Document::load` 逐个打开 → 逐页 `pages.append()` → `save`

   **拆分**:
   ```rust
   #[tauri::command]
   async fn split_pdf(file: String, ranges: Vec<String>, output_dir: String) -> Result<Vec<String>, String>
   ```
   - `ranges` 示例:`["1-5", "6-10", "11-"]`
   - 每个 range 输出一个 PDF

   **压缩**:
   ```rust
   #[tauri::command]
   async fn compress_pdf(file: String, output_path: String) -> Result<String, String>
   ```
   - 遍历页面的 XObject → 找图片 → `image` crate 重采样降低质量 → 写回

3. 在 `lib.rs` 注册三个命令。

4. **与成员 E 的接口约定**(在写前端前和他对齐):
   ```
   合并: 输入文件列表+输出路径 → 返回成功/失败
   拆分: 输入文件+页码范围数组+输出目录 → 返回生成的文件名列表
   压缩: 输入文件+输出路径 → 返回原大小 vs 新大小
   ```

**验收**:
- [ ] 合并 3 个 PDF 为 1 个,页序正确
- [ ] 按 `["1-3","4-6"]` 拆分,输出 2 个文件页码范围对
- [ ] 压缩后体积下降且内容完整

---

### B-3 — Office 文档转换(Step 23)

**做什么**:

1. 编写 `src-tauri/src/commands/office.rs`:
   ```rust
   #[tauri::command]
   async fn convert_office(input: String, target_format: String, output: String) -> Result<String, String>
   ```
   - 调 LibreOffice headless(子进程):
     ```
     soffice --headless --convert-to pdf --outdir output_dir input.docx
     ```
   - 支持:Word/Excel/PPT ↔ PDF,Word ↔ HTML
   - 复用成员 A 的 `common/dependency.rs` 机制:首次使用时检测 LibreOffice,缺失则引导安装

2. 在 `lib.rs` 注册命令。

**验收**:Word→PDF 排版基本保留,Excel 表格正确。

---

### B-4 — 批量解压/压缩(Step 25)

**做什么**:

1. 编写 `src-tauri/src/commands/archive.rs`:

   **批量解压**:
   ```rust
   #[tauri::command]
   async fn batch_extract(files: Vec<String>, output_dir: String) -> Result<Vec<String>, String>
   ```
   - zip/7z/tar.gz:用 `zip` crate 或调 7z 子进程
   - rar:用 `unrar` 子进程(需用户单独安装)
   - 每个文件解压到 `output_dir/{原文件名(无扩展名)}/`

   **批量打包**:
   ```rust
   #[tauri::command]
   async fn batch_compress(files: Vec<String>, format: String, password: Option<String>, output_dir: String) -> Result<Vec<String>, String>
   ```

2. 加依赖:`cd src-tauri && cargo add zip flate2`

**验收**:多格式批量解压;加密打包/解压正确。

---

## 四、协作点

| 你需要 | 从谁获取 | 时机 |
|---|---|---|
| `AppError`、`Task`、`Progress` 类型 | A(A-1) | B-1 开始前 |
| `BatchRunner` worker 框架 | A(A-2) | B-1 开始前 |
| `FileDropZone.vue`、`TaskProgress.vue` | D(D-2) | B-1 前端部分 |
| 前端基础布局(侧边栏路由) | D(D-1) | B-1 前端部分 |
| PDF 前端页面 | **你给 E 接口** | B-2 完成前 |

---

## 五、验收总清单

- [ ] B-1:图片批量压缩全栈跑通,进度+对比+取消
- [ ] B-2:PDF 合并/拆分/压缩后端三个命令正确
- [ ] B-3:Office→PDF 转换正确
- [ ] B-4:批量解压/打包,加密支持

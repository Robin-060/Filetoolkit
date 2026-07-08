# 成员 C 任务卡 — 重命名查重音频OCR

> **你的角色**:文件重命名、查重、音频、OCR 的开发。两个 M1 核心功能全栈,外加 M2/M4 的特色模块。
> **代号**:RenameDedup

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
| C-1 | 文件批量重命名(全栈) | M1 | A-1, A-2, D-1 | B 同时做 B-1 |
| C-2 | 重复文件查重(后端) | M1 | A-1, A-2 | E 同时做查重前端 |
| C-3 | 音频处理模块 | M2 | A-3 | 可与 A-4 并行 |
| C-4 | PDF OCR | M4 | A-3 的依赖机制 | 可与 B-3/B-4 并行 |
| C-5 | 文件校验(MD5/SHA) | M4 | 无 | 可与 C-4 并行 |

---

## 三、每步详解

### C-1 — 文件批量重命名(Step 6,全栈)

**后端 `commands/rename.rs`**:

1. 加依赖:`cd src-tauri && cargo add chrono trash`

2. 实现模板解析:
   ```rust
   fn parse_pattern(pattern: &str, file_path: &Path, index: usize) -> Result<String, String> {
       // {name} → 原文件名(不含扩展名)
       // {ext} → 扩展名
       // {index} / {index:3} → 序号(补齐 3 位)
       // {date} / {date:yyyy-MM-dd} → 文件修改日期
       // 用 regex 替换 pattern 中的占位符
   }
   ```

3. 实现两个命令:

   **预览**(不实际改名):
   ```rust
   #[tauri::command]
   fn preview_rename(files: Vec<String>, pattern: String) -> Result<Vec<RenamePreview>, String>
   ```
   - 返回 `[{old_name, new_name, conflict: bool}]`
   - `conflict` 检测:新名是否与表中其他新名或已有文件重名

   **执行**:
   ```rust
   #[tauri::command]
   async fn execute_rename(app: AppHandle, plan: Vec<{old: String, new: String}>) -> Result<Vec<String>, String>
   ```
   - 用 `trash::delete` 移到回收站(可撤销),而非直接覆盖
   - emit 进度事件

4. 注册两个命令到 `lib.rs`。

**前端 `pages/RenamePage.vue`**:

5. UI 布局:
   - 拖入文件列表(显示原名)
   - 模板输入框 + 快捷按钮:`{name}` `{ext}` `{index:3}` `{date:yyyy-MM-dd}`
   - **实时预览区**:每行显示 原名 → 新名,绿色=OK,红色=冲突
   - [应用]按钮:确认执行
   - [撤销]按钮:恢复上一次改名

6. 调用流程:
   ```
   用户输入模板 → invoke("preview_rename", {files, pattern}) → 展示预览
   用户点应用 → invoke("execute_rename", {plan}) → 进度条 → 完成
   用户点撤销 → 从回收站恢复
   ```

**验收**:
- [ ] `{name}-v2-{index:3}` 预览正确,序号补零
- [ ] 冲突项标红,阻止执行
- [ ] 执行后文件确实改名
- [ ] 撤销能恢复

---

### C-2 — 重复文件查重(后端)(Step 7)

> ⚠️ 这步你只写**后端**,前端由成员 E 负责。

**后端 `commands/dedup.rs`**:

1. 加依赖:`cd src-tauri && cargo add blake3`

2. 实现扫描命令:
   ```rust
   #[tauri::command]
   async fn scan_duplicates(app: AppHandle, dir: String) -> Result<Vec<DuplicateGroup>, String>
   ```
   - **第一阶段**:遍历目录,按文件大小分组,只保留同大小且 > 1 的组
   - **第二阶段**:对同大小组,取前 8KB 快速 hash → 若相同再 hash 全文件(`blake3` 极快)
   - **第三阶段**:哈希相同的归为一组 → emit 进度
   - 用 `rayon` 并行 hash

3. 实现删除命令:
   ```rust
   #[tauri::command]
   async fn delete_duplicates(keep_strategy: String, groups: Vec<DuplicateGroup>) -> Result<u32, String>
   ```
   - `keep_strategy`:`"newest"` / `"largest"` / `"first"`
   - 每组保留一个,其余移到回收站

4. 数据结构:
   ```rust
   #[derive(Serialize)]
   struct DuplicateGroup {
       files: Vec<FileInfo>,  // 重复文件列表
   }
   #[derive(Serialize)]
   struct FileInfo {
       path: String, size: u64, modified: u64, hash: String,
   }
   ```

5. **性能注意**:大目录用 `walkdir` 流式遍历,先按大小筛掉大部分,避免对大文件全部 hash。

6. 注册两个命令到 `lib.rs`。

**验收**:
- [ ] 扫描含重复图片的目录,正确分组
- [ ] "保留最新"策略:最旧文件被删,最新保留
- [ ] 大目录有进度,可取消
- [ ] 1000 文件(总 1GB) < 30 秒

---

### C-3 — 音频处理模块(Step 14)

**做什么**:

1. 编写 `src-tauri/src/commands/audio.rs`:

   **格式转换**:
   ```rust
   #[tauri::command]
   async fn convert_audio(input: String, format: String, bitrate: String, output: String) -> Result<String, String>
   ```
   - `ffmpeg -i input -codec:a libmp3lame -b:a 320k output.mp3`
   - 支持 MP3/AAC/FLAC/WAV/OGG

   **剪切**:
   ```rust
   #[tauri::command]
   async fn cut_audio(input: String, start: String, end: String, output: String)
   ```

   **合并**:
   ```rust
   #[tauri::command]
   async fn merge_audio(files: Vec<String>, output: String)
   ```

   **音量标准化**(EBU R128):
   ```rust
   #[tauri::command]
   async fn normalize_audio(input: String, target_lufs: f64, output: String)
   ```
   - `ffmpeg -i input -af loudnorm=I=-16:TP=-1.5:LRA=11 output`

2. 注册命令到 `lib.rs`。

**前端**:成员 E 有空时写 `pages/AudioPage.vue`,你先保证后端命令正确。

**验收**:转换无损,剪切精确,标准化后响度统一。

---

### C-4 — PDF OCR(Step 24)

**做什么**:

1. 编写 `src-tauri/src/commands/ocr.rs`:
   ```rust
   #[tauri::command]
   async fn ocr_pdf(input: String, languages: Vec<String>, output: String) -> Result<String, String>
   ```
   - 调 Tesseract 子进程:`tesseract input.png output -l chi_sim+eng pdf`
   - 流程:PDF → 逐页渲染为图片 → OCR → 生成双层 PDF(图片层+文字层)
   - 复用成员 A 的 `common/dependency.rs` 机制:检测 Tesseract,缺失则引导安装

2. 加 PDF 渲染依赖(把 PDF 页转图片):可调 `poppler` 或 `mupdf` 命令行工具,或 Rust 绑定

**验收**:清晰扫描件中英文识别率 > 90%,输出可搜索 PDF。

---

### C-5 — 文件校验(Step 26)

**做什么**:

1. 编写 `src-tauri/src/commands/checksum.rs`:

   ```rust
   #[tauri::command]
   async fn compute_checksum(app: AppHandle, file: String, algorithm: String) -> Result<String, String>
   ```
   - algorithm:`"md5"` / `"sha1"` / `"sha256"` / `"blake3"`
   - 大文件流式读(4MB buffer),emit 进度

   ```rust
   #[tauri::command]
   async fn verify_checksum(file: String, expected: String, algorithm: String) -> Result<bool, String>
   ```

   ```rust
   #[tauri::command]
   async fn batch_verify(files: Vec<{path: String, expected: String, algo: String}>) -> Result<Vec<VerifyResult>, String>
   ```

2. 注册命令。

**验收**:大文件哈希有进度,比对结果清晰。

---

## 四、协作点

| 你需要 | 从谁获取 | 时机 |
|---|---|---|
| `AppError`、`Task` 类型 | A(A-1) | C-1 开始前 |
| `BatchRunner` worker 框架 | A(A-2) | C-1 开始前 |
| `FileDropZone.vue`、`TaskProgress.vue` | D(D-2) | C-1 前端部分 |
| 前端基础布局 | D(D-1) | C-1 前端部分 |
| ffmpeg 依赖探测机制 | A(A-3) | C-3 开始前 |
| 查重前端页面 | **你给 E 接口** | C-2 完成前 |

---

## 五、验收总清单

- [ ] C-1:重命名模板预览+冲突检测+撤销
- [ ] C-2:查重两阶段扫描,智能保留,1000 文件 < 30s
- [ ] C-3:音频转换/剪切/合并/标准化
- [ ] C-4:OCR 双层 PDF,中英文 > 90%
- [ ] C-5:MD5/SHA 校验,大文件进度条

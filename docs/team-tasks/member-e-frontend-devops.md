# 成员 E 任务卡 — 前端功能页 + DevOps + 发布

> **你的角色**:前端功能页面 + CI/CD + 打包发布。与 B/C 配合把功能页面写出来,同时把工程化基础设施(CI、测试、打包)全部搞定。
> **代号**:FrontendDevOps

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
| E-1 | PDF 功能前端页面 | M1 | B-2(后端), D-1(布局) | B 同时做后端 |
| E-2 | 查重功能前端页面 | M1 | C-2(后端), D-1(布局) | C 同时做后端 |
| E-3 | 视频/音频前端页面 | M2 | A-4/C-3(后端) | — |
| E-4 | CI/CD 流水线 | M2 | 无 | **尽早做,可与 E-1 并行** |
| E-5 | 自动化测试基线 | M2 | E-4 | 与 E-4 串联 |
| E-6 | 打包与分发 | M5 | D-6/D-7 | — |
| E-7 | 隐私与安全审计 | M5 | 所有功能完成 | 与 E-6/E-8 并行 |
| E-8 | 文档站与正式发布 | M5 | 所有功能完成 | 与 E-7 并行 |

---

## 三、每步详解

### E-1 — PDF 功能前端页面(Step 5 前端)

> 成员 B 写后端 `commands/pdf.rs`,你写前端 `pages/PdfPage.vue`,同时开工。

**做什么**:

1. 创建 `src/pages/PdfPage.vue`:

   **布局**:Tab 切换(合并 / 拆分 / 压缩)

   **合并模式**:
   - 拖入多个 PDF(用 D 的 `FileDropZone`,accept=".pdf")
   - 拖拽排序列表(上下箭头调整顺序)
   - 输出文件名输入框
   - [合并]按钮 → `invoke("merge_pdfs", {files, outputPath})` → 进度条 → 结果

   **拆分模式**:
   - 选一个 PDF 文件
   - 页码范围输入框:每行一个范围(如 `1-5`、`6-10`、`11-`)
   - 输出目录选择
   - [拆分]按钮 → `invoke("split_pdf", {file, ranges, outputDir})` → 生成文件列表

   **压缩模式**:
   - 拖入 PDF
   - 显示原文件大小
   - [压缩]按钮 → `invoke("compress_pdf", {file, outputPath})` → 进度条 → 原大小 vs 新大小

2. **调用约定**(与 B 对齐):
   ```
   merge_pdfs:   files: string[], outputPath: string → string(结果消息)
   split_pdf:    file: string, ranges: string[], outputDir: string → string[](输出文件路径列表)
   compress_pdf: file: string, outputPath: string → {originalSize: number, newSize: number}
   ```

**验收**:
- [ ] 三个模式 Tab 切换流畅
- [ ] 合并:拖入 3 个 PDF,排序,合并成功
- [ ] 拆分:输入 `1-3,4-6`,输出 2 个文件
- [ ] 压缩:显示体积下降

---

### E-2 — 查重功能前端页面(Step 7 前端)

> 成员 C 写后端 `commands/dedup.rs`,你写前端 `pages/DedupPage.vue`。

**做什么**:

1. 创建 `src/pages/DedupPage.vue`:

   **UI 流程**:
   ```
   选目录 → [开始扫描] → 进度条(扫描中...发现 N 组重复)
   → 结果列表(每组可展开,显示文件列表+大小+修改日期)
   → 保留策略下拉(保留最新/最大/第一个)
   → [删除选中] 按钮 → 确认对话框 → 执行
   ```

2. **调用后端**:
   ```typescript
   // 扫描
   const groups = await invoke("scan_duplicates", { dir: "/path/to/scan" });
   // groups: [{files: [{path, size, modified, hash}, ...]}, ...]

   // 删除
   const deletedCount = await invoke("delete_duplicates", {
     keepStrategy: "newest",
     groups: selectedGroups
   });
   ```

3. **结果展示**:每组显示为卡片,展开后每行:文件名+大小+修改日期,可逐个勾选

**验收**:
- [ ] 扫描有进度,结果分组正确
- [ ] 保留策略生效
- [ ] 可取消扫描

---

### E-3 — 视频/音频前端页面(Step 12/14 前端)

**做什么**:

1. **`pages/VideoPage.vue`**(对接 A-4/A-5):

   三个 Tab:**剪切** / **转码** / **GIF**

   **剪切模式**:
   - 选视频文件 → 显示时长 → 输入开始/结束时间(HH:MM:SS 或秒数)
   - 输出格式选择(保持原格式/MP4)
   - [剪切]按钮

   **转码模式**:
   - 选视频 → 目标格式下拉(MP4/MOV/MKV/WebM)
   - 编码选择(H.264/H.265) + 质量/码率滑块
   - GPU 加速开关(后端检测后显示可用选项)

   **GIF 模式**:
   - 选视频 → 开始时间+持续时长 → 帧率(默认 10fps) → 宽度(默认 320px)
   - [生成 GIF]按钮

2. **`pages/AudioPage.vue`**(对接 C-3):

   两个 Tab:**格式转换** / **剪切合并**

   **格式转换**:选文件 → 目标格式(MP3/AAC/FLAC/WAV) → 码率 → [转换]

   **剪切**:选文件 → 开始/结束时间 → [剪切]

**验收**:视频/音频各模式功能正确。

---

### E-4 — CI/CD 流水线(Step 15)

> 🚀 **这步可以最早做**,不依赖任何功能代码。

**做什么**:

1. 创建 `.github/workflows/ci.yml`:

```yaml
name: CI

on:
  push:
    branches: [main]
  pull_request:
    branches: [main]

jobs:
  frontend:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: actions/setup-node@v4
        with: { node-version: "22" }
      - uses: pnpm/action-setup@v4
        with: { version: "9" }
      - run: pnpm install
      - run: pnpm lint
      - run: pnpm build

  backend:
    runs-on: windows-latest  # Tauri 需要 Windows 编译
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - run: cargo fmt --manifest-path src-tauri/Cargo.toml --check
      - run: cargo clippy --manifest-path src-tauri/Cargo.toml -- -D warnings
      - run: cargo test --manifest-path src-tauri/Cargo.toml
```

2. 验证:提一个 PR,确认 CI 能跑起来。

**验收**:PR 合并前 CI 全部绿灯。

---

### E-5 — 自动化测试基线(Step 16)

**做什么**:

1. **Rust 单元测试**:在每个 `commands/xxx.rs` 中添加 `#[cfg(test)] mod tests { ... }`
   - 核心逻辑必须有测试(如模板解析、哈希比对、页码范围解析)
   - 目标覆盖率 > 60%

2. **前端测试**:`pnpm add -D vitest @vue/test-utils jsdom`
   - 关键组件(FileDropZone、TaskProgress)写渲染测试
   - composables 写逻辑测试

3. CI 中集成测试(已在 Step 15 的 CI 配置中加了 `cargo test`)

**验收**:CI 自动跑测试,失败拦截。

---

### E-6 — 打包与分发(Step 30)

**做什么**:

1. **Tauri 打包配置**(`src-tauri/tauri.conf.json`):
   - 应用名、标识符、图标、窗口配置
   - `bundle`:Windows 用 msi/nsis,Mac 用 dmg,Linux 用 deb/AppImage
   - 签名配置(Windows 代码签名证书 / Mac 开发者证书)

2. **便携版构建脚本**(`scripts/build-portable.ps1`):
   - Windows:`pnpm tauri build` → 把 exe 和依赖打包成 zip,免安装可运行

3. **可选模块按需下载机制**:
   - ffmpeg、LibreOffice、Tesseract 不打包进安装包
   - 应用首次使用时引导下载(成员 A 的 A-3 已实现框架)

**验收**:三端安装包正常;便携版免安装运行;核心包 < 30MB。

---

### E-7 — 隐私与安全审计(Step 31)

**做什么**:

1. **抓包验证**:用 Wireshark / Proxyman 监控,确认应用运行时除可选模块下载外**无任何对外网络请求**

2. **逐条核对**(对照 `docs/PRD.md` §4.3):
   - [ ] 文件处理全部本地完成
   - [ ] 默认零遥测(无统计、无 crash report 上传)
   - [ ] 日志/临时文件存放用户私有目录
   - [ ] "一键清除"按钮功能可用
   - [ ] 核心功能离线可用
   - [ ] EXIF 去除选项默认开启

3. **关闭所有调试/开发模式的后门**

4. 写审计报告:`docs/design/security-audit.md`

**验收**:抓包零意外请求;一键清除日志有效。

---

### E-8 — 文档站与正式发布(Step 32)

**做什么**:

1. **完善 README**:功能截图、完整特性列表、开发指南链接

2. **文档站**(可选,推荐用 VitePress):
   ```bash
   pnpm add -D vitepress
   npx vitepress init
   ```
   - 结构:首页 → 功能指南 → 开发文档 → 隐私声明 → FAQ

3. **CHANGELOG.md**:从 git log 整理每个版本的变更

4. **正式发布**:
   ```bash
   git tag v1.0.0
   git push --tags
   gh release create v1.0.0 --title "FileToolkit v1.0" --notes "首个正式版本"
   ```

**验收**:新用户照着文档能独立上手;GitHub Release 可下载安装包。

---

## 四、协作点

| 你需要 | 从谁获取 | 时机 |
|---|---|---|
| PDF 后端命令 `merge_pdfs` 等 | B(B-2) | E-1 开始前约定接口 |
| 查重后端命令 `scan_duplicates` 等 | C(C-2) | E-2 开始前约定接口 |
| 前端布局+路由+组件 | D(D-1/D-2) | E-1 开始前 |
| 视频后端命令 | A(A-4/A-5) | E-3 开始前约定接口 |
| 音频后端命令 | C(C-3) | E-3 开始前约定接口 |

---

## 五、验收总清单

- [ ] E-1:PDF 合并/拆分/压缩前端三个模式可用
- [ ] E-2:查重扫描+结果展示+智能保留删除
- [ ] E-3:视频剪切/转码/GIF + 音频转换/剪切
- [ ] E-4:CI 在 PR 时自动跑 lint+clippy+test
- [ ] E-5:核心逻辑测试覆盖率 > 60%
- [ ] E-6:三端安装包+便携版,核心包 < 30MB
- [ ] E-7:零遥测,审计报告通过
- [ ] E-8:文档站上线,Release 可下载

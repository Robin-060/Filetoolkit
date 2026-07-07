# 队友上手指南 — FileToolkit · 全里程碑版

> 从环境搭建到全部 5 个里程碑(M1→M5)的完整开发步骤。带着这份文档,能独立把整个项目做完。
> 每步都标注了"可并行"标记和分工建议,方便多人协作。

| 项目     | 内容                                         |
| -------- | -------------------------------------------- |
| 文档版本 | v2.0                                         |
| 创建日期 | 2026-07-07                                   |
| 总步数   | **32 步**(M1~M5,不含已完成的 M0)             |
| 当前阶段 | M0 ✅ / M1 ⏳ / M2-M5 ⏸                      |

---

## 一、快速开始(15 分钟)

### 1.1 下载代码

```bash
git clone https://github.com/Robin-060/file-toolkit.git
cd file-toolkit
```

> 如果不用 git,也可从 GitHub 下载 ZIP 解压。

### 1.2 环境要求

| 工具 | 版本要求 | 安装方式 | 验证命令 |
| --- | --- | --- | --- |
| Node.js | ≥ 18 | [nodejs.org](https://nodejs.org) | `node --version` |
| pnpm | ≥ 8 | `npm i -g pnpm` | `pnpm --version` |
| Rust | stable | [rustup.rs](https://rustup.rs) | `rustc --version` |
| **Windows 额外** | MSVC Build Tools | winget 或 [VS 官网](https://visualstudio.microsoft.com/downloads/#build-tools-for-visual-studio-2022) | 装好后终端里 `where link` 能看到 MSVC 版 |

> ⚠️ Windows 上 MSVC Build Tools 安装时要勾选 **"使用 C++ 的桌面开发"** 工作负载,否则 Rust 编译报 `linker 'link.exe' not found`。

### 1.3 跑通项目

```bash
pnpm install
pnpm tauri dev
```

**预期**:弹窗 → 输入"世界" → 点按钮 → 显示 `来自 Rust 后端的问候:你好,世界!` ✅

---

## 二、项目结构速览

```
file-toolkit/
├── docs/                         ← 所有文档(.md)
│   ├── CONTRIBUTING.md           # 本文档
│   ├── PRD.md                    # 产品需求(功能详情)
│   ├── ROADMAP.md                # 宏观路线图
│   └── AGENTS.md                 # 编码与协作规范
│
├── src/                          ← Vue 3 前端(TypeScript)
│   ├── App.vue                   # 根组件(已有 greet 示例)
│   ├── pages/                    # 功能页面(★ 主要开发区)
│   ├── components/               # 通用 UI 组件
│   ├── composables/              # 组合式函数
│   └── store/                    # Pinia 状态
│
├── src-tauri/                    ← Rust 后端
│   └── src/
│       ├── commands/             # 功能命令(★ 主要开发区)
│       ├── pipeline/             # 流水线引擎(M3)
│       ├── worker/               # 任务队列+进度
│       └── common/               # 共享类型/错误
│
├── Cargo.toml                    # workspace 清单
├── package.json
└── .gitignore                    # 已配好,不要改
```

### 核心通信模式

```
前端: invoke("命令名", {参数})  ──►  Rust: #[tauri::command] fn 命令名(...)
前端: 接收返回值 / 事件         ◄──  Rust: return / emit("事件", 数据)
```

---

## 三、开发工作流

每做一个功能,标准流程:

1. **读需求** → PRD.md 对应章节
2. **写后端** → `commands/xxx.rs` 实现 `#[tauri::command]`
3. **注册命令** → `lib.rs` 的 `generate_handler![]` 加一行
4. **写前端** → `pages/XxxPage.vue`,用 `invoke` 调用
5. **调通** → `pnpm tauri dev` 启动测试
6. **提交** → `git commit -m "feat: xxx"`

---

## 四、全部任务清单(M1 → M5,共 32 步)

> 每步格式:**目标** → **可并行?** → **谁做?** → **要做什么** → **产出** → **验收**

---

### ◆ M1 — MVP 核心功能(Step 1~10,当前阶段)

#### Step 1 — 统一错误处理与共享类型

| 维度 | 内容 |
| --- | --- |
| **可并行** | 否,**必须最先做**(后续所有步骤依赖) |
| **建议谁做** | 后端主力(熟悉 Rust) |
| **要做什么** | ① `common/error.rs`:用 `thiserror` 定义错误枚举(文件不存在/IO 错误/格式不支持/任务取消等),实现 `serde::Serialize` 可传给前端<br>② `common/types.rs`:定义 `Task {id, input_path, output_path, status, progress}`、`Progress {current, total}`、`TaskStatus` 枚举 |
| **产出** | `common/error.rs`、`common/types.rs` |
| **验收** | 错误可序列化为 JSON;类型被后续模块引用 |

---

#### Step 2 — 任务队列与进度上报框架

| 维度 | 内容 |
| --- | --- |
| **可并行** | 否,依赖 Step 1 |
| **建议谁做** | 后端主力 |
| **要做什么** | ① `worker/mod.rs`:接收 `Vec<Task>` + 处理闭包 → `rayon` 多核并行 → `tauri::Emitter` 回传进度<br>② `Arc<AtomicBool>` 实现取消信号<br>③ 失败隔离:单 task 失败记录错误,不中断其余 |
| **产出** | `worker/mod.rs` |
| **验收** | 100 个模拟 task 并行跑,前端有进度;单个失败不中断;可取消 |

---

#### Step 3 — 前端基础设施

| 维度 | 内容 |
| --- | --- |
| **可并行** | ✅ **可与 Step 1-2 并行**(前后端不冲突) |
| **建议谁做** | 前端主力 |
| **要做什么** | ① 装 Pinia + Element Plus + Vue Router<br>② `store/task.ts`:任务状态管理<br>③ `components/AppLayout.vue`:侧边栏+主区布局<br>④ 配置 4 个功能页路由占位<br>⑤ 装 ESLint + Prettier,配 `.prettierrc.json` 和 `eslint.config.js` |
| **产出** | `store/task.ts`、`components/AppLayout.vue`、路由配置、lint 配置 |
| **验收** | 侧边栏导航切换 4 页;`pnpm lint` 通过 |

---

#### Step 4 — 图片批量压缩/转换 ★

| 维度 | 内容 |
| --- | --- |
| **可并行** | ✅ **与 Step 5/6/7 并行**(4 个功能各自独立) |
| **建议谁做** | 队友 A(全栈:后端 `image.rs` + 前端 `ImagePage.vue`) |
| **要做什么** | **后端**:`compress_images(files, quality, format, output_dir)` → `image` crate 读→resize→编码→写,emit 进度<br>**前端**:拖拽区 + 格式/质量参数 + 进度条 + 体积对比<br>**注册**:`lib.rs` 加 `commands::image::compress_images` |
| **产出** | `commands/image.rs`、`pages/ImagePage.vue` |
| **验收** | 拖 50 张图压缩,进度实时,失败隔离,可取消 |

---

#### Step 5 — PDF 合并/拆分/压缩 ★

| 维度 | 内容 |
| --- | --- |
| **可并行** | ✅ 与 Step 4/6/7 并行 |
| **建议谁做** | 队友 B(全栈) |
| **要做什么** | **后端**:合并(`lopdf` 逐页拷贝)、拆分(按页码范围提取)、压缩(重采样图片)<br>**前端**:Tab 切换(合并/拆分/压缩),拖拽排序,页码范围输入<br>**依赖**:`cargo add lopdf` |
| **产出** | `commands/pdf.rs`、`pages/PdfPage.vue` |
| **验收** | 合并 3→1 页序正确;拆分页码范围对;压缩体积下降 |

---

#### Step 6 — 文件批量重命名 ★

| 维度 | 内容 |
| --- | --- |
| **可并行** | ✅ 与 Step 4/5/7 并行 |
| **建议谁做** | 队友 C(或 A 做完图片后接) |
| **要做什么** | **后端**:解析模板 `{name}` `{ext}` `{index:3}` `{date}` → 预览列表(含冲突检测)→执行重命名→支持撤销(`trash` crate)<br>**前端**:文件列表 + 模板输入 + 实时预览(绿=正常/红=冲突)<br>**依赖**:`cargo add chrono trash` |
| **产出** | `commands/rename.rs`、`pages/RenamePage.vue` |
| **验收** | 模板预览正确;冲突标红阻止;可撤销 |

---

#### Step 7 — 重复文件查重 ★

| 维度 | 内容 |
| --- | --- |
| **可并行** | ✅ 与 Step 4/5/6 并行 |
| **建议谁做** | 队友 D(或 B 做完 PDF 后接) |
| **要做什么** | **后端**:两阶段(大小预筛 → `blake3` 哈希) → 分组 → 智能保留(最新/最大/第一个)→ 删其余<br>**前端**:选目录→扫描进度→分组展示→保留策略下拉→一键删除<br>**依赖**:`cargo add blake3`<br>**性能注意**:大目录流式读,不要全加载进内存 |
| **产出** | `commands/dedup.rs`、`pages/DedupPage.vue` |
| **验收** | 正确识别重复;大目录有进度可取消;1000 文件 < 30s |

---

 #### Step 8 — 统一通用组件

| 维度 | 内容 |
| --- | --- |
| **可并行** | 与 Step 4-7 穿插进行(做完前两个功能就开始抽共性) |
| **建议谁做** | 前端主力 |
| **要做什么** | `FileDropZone.vue`(拖拽选文件)、`TaskProgress.vue`(进度条+取消)、`ResultList.vue`(结果展示)、`useBatchTask.ts`(封装 invoke+事件) |
| **产出** | 4 个组件/composable |
| **验收** | 4 个功能页都复用同一套组件 |

---

#### Step 9 — 集成测试与 Bug 修复

| 维度 | 内容 |
| --- | --- |
| **可并行** | 否,依赖 Step 4-8 全部完成 |
| **建议谁做** | 全员 |
| **要做什么** | 写测试清单 `design/m1-test-cases.md`,覆盖:正常/大批量/大文件/特殊路径/取消/深色模式。逐项跑,修 bug |
| **验收** | 测试全通过;1000+ 文件不崩溃 |

---

#### Step 10 — M1 发布 v0.1.0

| 维度 | 内容 |
| --- | --- |
| **建议谁做** | 项目负责人 |
| **要做什么** | 更新 README(截图+状态),`CHANGELOG.md`,`git tag v0.1.0-alpha` |
| **验收** | 别人 clone 后 `pnpm tauri dev` 能跑 |

---

### ◆ M2 — 视频/音频处理 + 工程化(Step 11~16)

#### Step 11 — ffmpeg 可选依赖管理

| 维度 | 内容 |
| --- | --- |
| **可并行** | ✅ **与 M1 收尾(Step 8-10)并行** |
| **建议谁做** | 后端主力 |
| **要做什么** | `common/dependency.rs`:启动时探测 ffmpeg(优先 PATH→其次 bundled)→缺失弹引导→结果缓存<br>下载脚本:`scripts/download-ffmpeg.ps1`(Win)/`.sh`(Mac/Linux) |
| **产出** | `common/dependency.rs`、下载脚本 |
| **验收** | 有 ffmpeg 时秒过,无时弹出引导;缓存后重启不重复探测 |

---

#### Step 12 — 视频剪切/转码

| 维度 | 内容 |
| --- | --- |
| **可并行** | ✅ **与 Step 14(音频)并行** |
| **建议谁做** | 队友 A |
| **要做什么** | **后端**:`cut_video`(时间段裁剪,`-c copy` 无损模式)、`transcode_video`(格式互转,H.264/H.265)<br>解析 ffmpeg stderr 取时间进度 → emit<br>**前端**:`pages/VideoPage.vue`,时间轴选择,格式/编码下拉,进度条 |
| **产出** | `commands/video.rs`、`pages/VideoPage.vue` |
| **验收** | 无损剪切秒级;转码输出正确;进度按时间百分比 |

---

#### Step 13 — 硬件加速 + 压缩 + GIF

| 维度 | 内容 |
| --- | --- |
| **可并行** | 依赖 Step 12 |
| **建议谁做** | 队友 A(继续) |
| **要做什么** | NVENC/QSV/VideoToolbox 自动检测;目标码率压缩;截取片段生成 GIF(可调帧率/尺寸);从视频提取音轨 |
| **验收** | N 卡自动用 NVENC 加速;GIF 帧率可调;音频提取正确 |

---

#### Step 14 — 音频处理模块

| 维度 | 内容 |
| --- | --- |
| **可并行** | ✅ **与 Step 12/13 并行(不同队友)** |
| **建议谁做** | 队友 B |
| **要做什么** | **后端** `commands/audio.rs`:格式转换(MP3/AAC/FLAC/WAV)、剪切合并、音量标准化(`loudnorm`)<br>**前端** `pages/AudioPage.vue` |
| **产出** | `commands/audio.rs`、`pages/AudioPage.vue` |
| **验收** | 格式转换无损;剪切精确;标准化后响度统一 |

---

#### Step 15 — CI/CD 流水线

| 维度 | 内容 |
| --- | --- |
| **可并行** | ✅ **与 M2 任何步骤并行**(越早越好) |
| **建议谁做** | 任意队友(熟悉 GitHub Actions) |
| **要做什么** | `.github/workflows/ci.yml`:push/PR 时跑 `pnpm lint` + `cargo fmt --check` + `cargo clippy` + `cargo test` + `pnpm build` |
| **验收** | PR 合并前所有检查必须绿 |

---

#### Step 16 — 自动化测试基线

| 维度 | 内容 |
| --- | --- |
| **可并行** | 依赖 Step 15 的 CI 框架,可与 M2 功能穿插写 |
| **建议谁做** | 写功能的人顺手写测试 |
| **要做什么** | Rust 单元测试(`#[cfg(test)]`)、前端 vitest、核心逻辑覆盖率 > 60% |
| **验收** | CI 中自动跑测试,失败拦截 |

---

### ◆ M3 — 流水线引擎(核心差异化)(Step 17~22)

> ⚠️ **必须等 M1 四个原子功能完成后再做**,否则流水线没东西可串联。

#### Step 17 — 流水线数据模型与执行引擎

| 维度 | 内容 |
| --- | --- |
| **可并行** | ✅ **与 Step 18(节点注册)可部分并行**(先定接口) |
| **建议谁做** | 后端主力(Rust 架构) |
| **要做什么** | `pipeline/model.rs`:定义 `Pipeline {nodes, edges}` / `Node {id, type, params}` / `Edge {from, to}`<br>`pipeline/executor.rs`:DAG 拓扑排序 → 按序调度 → `rayon` 并行同层节点 → 变量在节点间传递<br>失败策略:停止 / 跳过当前继续 |
| **产出** | `pipeline/model.rs`、`pipeline/executor.rs` |
| **验收** | JSON 描述的流水线能拓扑排序;串行+并行混合执行正确;变量传递正确 |

---

#### Step 18 — 节点类型注册表

| 维度 | 内容 |
| --- | --- |
| **可并行** | ✅ 与 Step 19(可视化编辑器)并行 |
| **建议谁做** | 后端主力(或 Step 17 同一人连续做) |
| **要做什么** | `pipeline/registry.rs`:把 M1/M2 所有功能注册为节点(图片压缩/PDF合并/重命名/查重/视频转码/音频转换……),每个节点有 input/output schema |
| **验收** | 每个 MVP 功能都能作为节点被流水线调用 |

---

#### Step 19 — 流水线可视化编辑器

| 维度 | 内容 |
| --- | --- |
| **可并行** | ✅ 与 Step 18 并行(前端不依赖后端注册表细节) |
| **建议谁做** | 前端主力 |
| **要做什么** | `pipeline/PipelineEditor.vue`:节点式拖拽编排 UI(基于 Vue Flow 或自研)。节点可拖拽、连线、删除、配置参数。实时校验(环检测、必填参数)。 |
| **验收** | 拖拽加节点→连线→配参数→点运行;有环时阻止执行并提示 |

---

#### Step 20 — 流水线模板系统

| 维度 | 内容 |
| --- | --- |
| **可并行** | 依赖 Step 19 |
| **建议谁做** | 前端主力(继续) |
| **要做什么** | 内置模板("照片整理归档"、"合同 PDF 标准化"等),保存/导入/导出 JSON;模板管理 UI |
| **验收** | 一键加载模板,导出再导入正确还原 |

---

#### Step 21 — Dry-run 预览与执行控制

| 维度 | 内容 |
| --- | --- |
| **可并行** | 依赖 Step 17 |
| **建议谁做** | 前后端配合 |
| **要做什么** | `pipeline/preview.rs`:执行前生成完整处理计划(列出所有文件及变换链)。前端:预览展示 + 暂停/继续/取消按钮 |
| **验收** | dry-run 展示完整计划;执行中可暂停,恢复后状态一致 |

---

#### Step 22 — 流水线实战打磨

| 维度 | 内容 |
| --- | --- |
| **建议谁做** | 全员 |
| **要做什么** | 跑通 3-5 个真实场景,优化体验,修边界问题 |
| **验收** | 真实场景下 2 分钟内编排并跑完一条流水线 |

---

### ◆ M4 — 高级功能(可选模块)(Step 23~27)

> M4 的功能都做成**可选模块**(按需下载依赖),不增大核心包体积。

#### Step 23 — Office 文档转换

| 维度 | 内容 |
| --- | --- |
| **可并行** | ✅ **与 Step 24/25/26/27 全部可并行** |
| **建议谁做** | 队友 A |
| **要做什么** | `commands/office.rs`:调用 LibreOffice headless → Word/Excel/PPT ↔ PDF ↔ HTML<br>复用 Step 11 的依赖管理机制,首次使用时检测→引导安装 |
| **验收** | Word→PDF 排版基本保留;Excel 表格正确 |

---

#### Step 24 — PDF OCR

| 维度 | 内容 |
| --- | --- |
| **可并行** | ✅ 与 Step 23/25/26/27 并行 |
| **建议谁做** | 队友 B |
| **要做什么** | `commands/ocr.rs`:集成 Tesseract(可选依赖),扫描件/图片 PDF → 可搜索双层 PDF(图片层+文字层),中英文 |
| **验收** | 清晰扫描件识别率 > 90% |

---

#### Step 25 — 批量解压/压缩

| 维度 | 内容 |
| --- | --- |
| **可并行** | ✅ |
| **建议谁做** | 队友 C |
| **要做什么** | `commands/archive.rs`:zip/7z/tar.gz 批量解压;批量打包(可选密码)。rar 需 unrar 二进制 |
| **验收** | 多格式批量解压;加密打包/解压正确 |

---

#### Step 26 — 文件校验

| 维度 | 内容 |
| --- | --- |
| **可并行** | ✅ |
| **建议谁做** | 队友 D(或 C 连续做) |
| **要做什么** | `commands/checksum.rs`:MD5/SHA1/SHA256 计算,与预期值比对,批量校验,大文件进度条 |
| **验收** | 大文件哈希有进度;比对结果清晰 |

---

#### Step 27 — 磁盘占用可视化

| 维度 | 内容 |
| --- | --- |
| **可并行** | ✅ |
| **建议谁做** | 前端主力 |
| **要做什么** | `commands/diskusage.rs`:扫描目录→返回体积树→前端用图表库(ECharts)渲染旭日图/树状图 |
| **验收** | 10GB+ 目录扫描秒级;图表层级清晰 |

---

### ◆ M5 — 打磨与发布(Step 28~32)

#### Step 28 — 国际化 i18n

| 维度 | 内容 |
| --- | --- |
| **可并行** | ✅ **与 Step 29(主题)并行** |
| **建议谁做** | 前端主力 |
| **要做什么** | 接入 `vue-i18n`,提取所有 UI 文案,提供简中/英双语。`locales/zh-CN.json` + `en.json` |
| **验收** | 切换语言后所有 UI 正确;加新语言只需加 json |

---

#### Step 29 — 主题与视觉打磨

| 维度 | 内容 |
| --- | --- |
| **可并行** | ✅ 与 Step 28 并行 |
| **建议谁做** | 前端+设计 |
| **要做什么** | 统一深/浅色主题变量;设计应用图标;统一组件样式;品牌规范 |
| **验收** | 深浅切换流畅;图标在 Win/Mac 任务栏清晰 |

---

#### Step 30 — 打包与分发

| 维度 | 内容 |
| --- | --- |
| **可并行** | 依赖 Step 28/29 |
| **建议谁做** | 项目负责人 |
| **要做什么** | `tauri.conf.json` 打包配置;Win/Mac/Linux 安装包 + 便携版;签名配置;可选模块按需下载机制 |
| **验收** | 三端安装包正常;便携版免安装运行;核心包 < 30MB |

---

#### Step 31 — 隐私与安全审计

| 维度 | 内容 |
| --- | --- |
| **可并行** | ✅ 与 Step 30/32 并行 |
| **建议谁做** | 项目负责人 |
| **要做什么** | 逐条核对 PRD §4.3 隐私红线:抓包确认无对外请求;验证"一键清除日志"功能;README 公开隐私承诺 |
| **验收** | 零遥测;所有处理本地;无隐蔽网络请求 |

---

#### Step 32 — 文档站与正式发布

| 维度 | 内容 |
| --- | --- |
| **建议谁做** | 全员 |
| **要做什么** | 完善 README + 用户指南 + FAQ + 隐私声明;可选用 VitePress 搭文档站;`git tag v1.0.0`;GitHub Release |
| **验收** | 新用户能照文档独立上手 |

---

## 五、并行开发与分工建议

### 5.1 依赖关系总图(谁必须在谁之后)

```
M0 ✅ 已完成
 │
 ├─ Step 1(错误/类型) ────┬─ Step 2(任务队列)
 │                        │
 │  Step 3(前端基建) ◄────┘  ← 可与 Step 1-2 并行
 │       │
 │       ├─ Step 4(图片) ──┐
 │       ├─ Step 5(PDF) ───┤
 │       ├─ Step 6(改名) ──┤ ← 四人可同时做这 4 个功能
 │       └─ Step 7(查重) ──┘
 │              │
 │       Step 8(通用组件)    ← 穿插进行
 │              │
 │       Step 9(测试) → Step 10(发布 v0.1.0)
 │
 ├─ M2 Step 11(ffmpeg依赖) ← 可与 M1 收尾并行
 │       │
 │       ├─ Step 12(视频) → Step 13(硬加速/GIF)
 │       ├─ Step 14(音频)   ← 与 Step 12 并行
 │       └─ Step 15(CI/CD)  ← 与任何步骤并行
 │
 ├─ M3 Step 17(引擎) + Step 18(注册表)
 │       │                    │
 │       └─ Step 19(可视化) ◄─┘ ← 前后端可并行
 │              │
 │       Step 20(模板) → Step 21(dry-run) → Step 22(打磨)
 │
 ├─ M4 Step 23/24/25/26/27   ← 五个全部可并行
 │
 └─ M5 Step 28 ∥ Step 29 → Step 30 ∥ Step 31 → Step 32
```

### 5.2 按团队规模的分工方案

#### 2 人团队

| 阶段 | 队友 A(后端为主) | 队友 B(前端为主) |
|---|---|---|
| M1 基础 | Step 1(错误/类型) + Step 2(任务队列) | Step 3(前端基建) ← 与 A 同时开工 |
| M1 功能 | Step 4(图片) + Step 5(PDF) 的后端 | Step 4+5 的前端,然后 Step 6(改名)+Step 7(查重)全栈 |
| M1 收尾 | Step 9(测试) | Step 8(组件) |
| M2 | Step 11(ffmpeg) + Step 12-14 后端 | Step 12-14 前端 + Step 15(CI) |
| M3 | Step 17-18 引擎+注册表 | Step 19-20 可视化编辑器+模板 |
| M4 | Step 23(Office) + Step 25(解压) | Step 24(OCR) + Step 26(校验) + Step 27(磁盘) |
| M5 | Step 30(打包) + Step 31(审计) | Step 28(国际化) + Step 29(主题) |

#### 3 人团队

| 阶段 | A(后端核心) | B(后端功能) | C(前端) |
|---|---|---|---|
| M1 基础 | Step 1+2 | Step 3 前端基建辅助 | Step 3 主力 |
| M1 功能 | 后端架构 review | Step 4+5 后端 | Step 4+5 前端 |
| M1 功能(续) | Step 6 后端 | Step 7 后端 | Step 6+7 前端 |
| M2 | Step 11+12+13 视频 | Step 14 音频 | 视频+音频前端,Step 15 CI |
| M3 | Step 17 引擎 | Step 18 注册表 | Step 19+20 可视化 |
| M4 | Step 23(Office) | Step 24+25(OCR+解压) | Step 26+27(校验+磁盘) |
| M5 | Step 30+31 | — | Step 28+29 |

#### 4 人团队(M1 最快)

| 阶段 | A | B | C | D |
|---|---|---|---|---|
| 基础 | Step 1+2 | Step 3 辅助 | Step 3 主力 | 熟悉代码,准备 Step 7 |
| 功能 | Step 4(图片) | Step 5(PDF) | Step 6(改名) | Step 7(查重) |
| 收尾 | A+C 测试 | B 组件 | D 组件 | — |
| M2 | 视频后端 | 音频 | 视频前端 | CI+测试 |

### 5.3 并行规则总结

| 规则 | 说明 |
|---|---|
| **Step 3 可与 Step 1-2 并行** | 前端基建不依赖后端类型/worker 定义(只要接口约定好) |
| **Step 4-7 四个功能可完全并行** | 各自独立的 `commands/xxx.rs` + `pages/XxxPage.vue`,零代码冲突 |
| **Step 15 CI 随时可做** | 建议最早接入,越早越受益 |
| **M4 五个功能全部可并行** | 都做成可选模块,互相不依赖 |
| **Step 28 和 29 可并行** | 国际化和主题不冲突 |
| **需要串行的地方** | Step 1→2, Step 11→12→13, Step 17→19→20→21 |

---

## 六、编码与协作规范

完整规则见 [`AGENTS.md`](./AGENTS.md),核心几条:

| 规则 | 说明 |
| --- | --- |
| 文档全部用 `.md` | 不生成 .docx/.pdf |
| 前端必须 TypeScript | 禁用纯 JS |
| Rust 公共 API 加 `///` | 必须文档化 |
| commit 前跑 lint | 前端 `pnpm lint`,后端 `cargo clippy && cargo fmt` |
| 文件操作三原则 | 失败隔离 + 可取消 + 预览(dry-run) |
| 默认不覆盖原文件 | 输出到独立目录或加后缀 |
| 隐私红线 | 文件不上传网络,零遥测 |
| commit 格式 | `feat:` `fix:` `docs:` `refactor:` |

---

## 七、常用命令

```bash
# === 开发 ===
pnpm tauri dev         # 启动完整环境
pnpm dev               # 仅前端 Vite

# === 后端 ===
cargo check --manifest-path src-tauri/Cargo.toml
cargo fmt --manifest-path src-tauri/Cargo.toml
cargo clippy --manifest-path src-tauri/Cargo.toml -- -D warnings

# === 前端 ===
pnpm lint
pnpm format
pnpm build

# === 加 Rust 依赖 ===
cd src-tauri && cargo add 包名

# === 加前端依赖 ===
pnpm add 包名
pnpm add -D 包名   # devDependency
```

---

## 八、FAQ

| 问题 | 解法 |
|---|---|
| rust-analyzer 报找不到项目 | Ctrl+Shift+P → "rust-analyzer: Restart server" |
| esbuild 报错 | `rm -rf node_modules pnpm-lock.yaml && pnpm install` |
| `linker 'link.exe' not found` | MSVC Build Tools 没装或没勾 C++ 负载 |
| 中文路径问题 | 用 `PathBuf` 而非 `String` 传路径 |
| 大文件崩溃 | 用 `BufReader` 流式读,不要全载入内存 |

---

有问题提 issue 或联系项目发起人。🚀

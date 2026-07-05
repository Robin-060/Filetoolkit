<div align="center">

# FileToolkit

**一站式本地文件批量处理工具**

本地优先 · 隐私安全 · 轻量高效 · 开源可审计

</div>

---

## 📖 项目简介

FileToolkit 是一个**完全在本地运行**的桌面文件批量处理工具。在一个应用内完成图片、文档、视频、音频、文件管理等绝大多数日常文件处理需求——**文件内容绝不离开你的电脑**。

对标市面上的在线压缩、在线 PDF 合并、在线视频转码等工具,但彻底解决三大痛点:

- 🔒 **隐私风险**:在线工具要求上传私人照片、合同、视频,无法确认是否被留存或泄露
- 🧰 **功能割裂**:图片、PDF、视频工具各自为政,需在多个网站间反复切换
- 🐌 **批量能力弱**:大多数在线工具只支持单文件,处理几百个文件效率极低

## ✨ 核心特性

| 特性 | 说明 |
| --- | --- |
| 🖥️ **本地优先** | 所有处理在用户设备本地完成,文件内容绝不离开本机 |
| 🔒 **隐私可证** | 开源可审计,默认零遥测,日志/临时文件可一键清除 |
| ⚡ **轻量高效** | 基于 Tauri 2,安装包仅几 MB,多核并行处理大批量文件 |
| 🧰 **一站聚合** | 图片 / 文档 / 视频 / 音频 / 文件管理,一个应用替代一抽屉小工具 |
| 🔗 **流水线串联** | 支持把多个操作串起来一次执行(核心差异化能力) |
| 🔄 **安全可控** | 操作前预览、操作后可撤销,失败不中断 |

## 🛠️ 技术栈

| 层级 | 技术 |
| --- | --- |
| 桌面框架 | [Tauri 2](https://tauri.app/) |
| 后端 | Rust |
| 前端 | Vue 3 + TypeScript + Vite |
| 视频/音频 | ffmpeg |
| 图片 | Rust `image` crate / libvips |
| PDF | lopdf / qpdf |
| 查重 | blake3 |

## 📋 项目状态

🚧 **早期开发中** —— 当前处于 M0(项目骨架)阶段,核心功能尚在规划。

完整路线图见 [`docs/ROADMAP.md`](./docs/ROADMAP.md),产品需求见 [`docs/PRD.md`](./docs/PRD.md)。

### 路线图概览

| 里程碑 | 名称 | 状态 |
| --- | --- | :--: |
| M0 | 项目骨架 | ✅ |
| M1 | MVP 核心功能(图片/PDF/重命名/查重) | ⏳ |
| M2 | 视频/音频处理 | ⏸ |
| M3 | 流水线引擎(灵魂功能) | ⏸ |
| M4 | 高级功能(Office/OCR/解压) | ⏸ |
| M5 | 打磨与发布 | ⏸ |

## 🚀 本地开发

### 环境要求

- [Node.js](https://nodejs.org/) ≥ 18
- [pnpm](https://pnpm.io/) ≥ 8
- [Rust](https://www.rust-lang.org/tools/install)(stable 工具链)
- **Windows**:MSVC Build Tools(C++ 工作负载)
- **macOS**:Xcode Command Line Tools
- **Linux**:参见 [Tauri 官方依赖说明](https://tauri.app/start/prerequisites/)

### 启动开发环境

```bash
# 安装前端依赖
pnpm install

# 启动开发模式(会自动编译 Rust 后端并弹出窗口)
pnpm tauri dev
```

### 常用脚本

```bash
pnpm dev              # 仅启动前端(Vite)
pnpm build            # 构建前端产物
pnpm tauri dev        # 启动完整开发环境(推荐)
pnpm tauri build      # 打包发布版本
pnpm lint             # 前端 lint
pnpm format           # 前端格式化
pnpm rust:fmt         # Rust 格式化
pnpm rust:clippy      # Rust 静态检查
```

## 📁 项目结构

```
file-toolkit/
├── docs/                # 项目文档(全部 Markdown)
│   ├── AGENTS.md        # AI 协作规范
│   ├── PRD.md           # 产品需求文档
│   ├── ROADMAP.md       # 实施路线图
│   └── design/          # 设计文档
├── src/                 # Vue 前端
├── src-tauri/           # Rust 后端
│   └── src/
│       ├── commands/    # 暴露给前端的功能命令
│       ├── pipeline/    # ★ 流水线引擎(M3)
│       ├── worker/      # 任务队列与进度上报
│       └── common/      # 共享类型与错误处理
└── README.md
```

## 🔒 隐私承诺

本项目主打**本地隐私安全**,以下为不可违反的原则:

- 🔒 任何文件内容**不得上传到网络**,所有处理在本地完成
- 🔒 默认**零遥测**;崩溃日志收集须显式同意、可关闭
- 🔒 临时文件/日志存放于用户私有目录,提供**一键清除**
- 🔒 不引入强制联网依赖,核心功能**离线可用**
- 🔒 EXIF/元数据默认提供去除选项

## 📄 开源协议

[MIT License](./LICENSE) © 2026 Robin-060

## 🤝 参与贡献

项目处于早期阶段,欢迎通过以下方式参与:

- 提交 [Issue](../../issues) 反馈 bug 或建议
- 在 Discussions 中参与功能讨论
- 提交 Pull Request(请先阅读 [`docs/AGENTS.md`](./docs/AGENTS.md) 了解协作规范)

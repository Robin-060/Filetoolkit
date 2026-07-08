# M0 — 项目骨架设计文档

> 本文档描述 FileToolkit 的 **M0 阶段(项目骨架)** 的目标、目录结构、初始化步骤与验收标准。
> M0 不是功能开发,而是为后续 M1(MVP)打好可运行、可维护、可扩展的地基。

| 项目     | 内容                  |
| -------- | --------------------- |
| 阶段     | M0(骨架 / Skeleton) |
| 文档版本 | v1.0                  |
| 创建日期 | 2026-07-02            |
| 状态     | 待执行                |

---

## 1. M0 目标

搭出一个**可运行的最小 Tauri 2 + Vue 3 骨架**,并跑通端到端通信闭环:

1. `pnpm tauri dev` 能成功启动并弹出桌面窗口。
2. 前端 Vue 页面有一个按钮,点击后**调用 Rust 后端命令**并展示返回结果。
3. 代码与文档目录符合 `AGENTS.md` §4 的约定。
4. 接入代码质量工具链(格式化 + lint),形成团队基线。
5. CI 配置就绪(可选,建议尽早接入)。

> **一句话:M0 完成 = 应用能跑、前后端能通、目录合规、lint 通过。**

---

## 2. 前置条件(环境就绪清单)

执行 M0 前,以下环境必须全部就绪:

| 工具 | 用途 | 验证命令 | 状态 |
| --- | --- | --- | --- |
| Node.js ≥ 18 | 前端运行时 + pnpm | `node --version` | ✅ v24.12.0 |
| pnpm | 包管理器 | `pnpm --version` | ✅ 11.1.1 |
| Git | 版本控制 | `git --version` | ✅ 2.54.0 |
| **MSVC Build Tools(C++ 工作负载)** | Rust 在 Windows 上的链接器 | `where link.exe`(MSVC 版) | ⏳ 安装中 |
| **Rust(rustup + stable)** | 后端编译 | `rustc --version` | ⏸ 待装 |
| WebView2 | Tauri 渲染 | Win10/11 通常预装 | ✅(Edge 自带) |

> ⚠️ **MSVC 是硬前置**:Rust 在 Windows 上必须用 MSVC 链接器。MSVC 装好前不要装 Rust。

---

## 3. 目录结构(M0 最终形态)

初始化后,我们将官方脚手架的结构**调整为本项目的约定**(对齐 `AGENTS.md` §4):

```
file-toolkit/
├── docs/                        # 所有文档(.md,见 AGENTS.md §2)
│   ├── AGENTS.md
│   ├── PRD.md
│   └── design/
│       └── m0-skeleton.md       # 本文件
│
├── src/                         # Vue 前端
│   ├── App.vue                  # 根组件
│   ├── main.ts                  # 入口
│   ├── pages/                   # 功能页面(图片/PDF/视频/重命名/查重)
│   │   └── Home.vue             # M0 占位页(含调通按钮)
│   ├── pipeline/                # 流水线可视化编辑器(M3 填充)
│   ├── store/                   # Pinia 状态管理
│   │   └── task.ts              # 任务状态(进度/队列)
│   ├── components/              # 通用 UI 组件
│   ├── composables/             # 组合式函数(如 useInvoke)
│   └── assets/
│
├── src-tauri/                   # Rust 后端
│   ├── src/
│   │   ├── main.rs              # 入口(tauri::Builder)
│   │   ├── lib.rs               # 库入口(供测试与移动端复用)
│   │   ├── commands/            # 暴露给前端的命令(每功能一模块)
│   │   │   ├── mod.rs           # 模块声明
│   │   │   ├── image.rs         # 图片处理(M1)
│   │   │   ├── pdf.rs           # PDF 处理(M1)
│   │   │   ├── video.rs         # 视频处理(M2)
│   │   │   ├── rename.rs        # 重命名(M1)
│   │   │   └── dedup.rs         # 查重(M1)
│   │   ├── pipeline/            # ★ 流水线引擎(M3)
│   │   │   └── mod.rs
│   │   ├── worker/              # 任务队列 + 进度上报
│   │   │   └── mod.rs
│   │   ├── common/              # 共享类型、错误、工具
│   │   │   ├── mod.rs
│   │   │   ├── error.rs         # 统一错误类型(thiserror)
│   │   │   └── types.rs         # 共享结构体(Task / Progress)
│   │   └── config.rs            # 应用配置
│   ├── Cargo.toml
│   ├── tauri.conf.json          # Tauri 配置(窗口/权限/标识)
│   ├── build.rs
│   └── icons/                   # 应用图标
│
├── scripts/                     # 构建/发布脚本
├── .editorconfig
├── .gitignore
├── .prettierrc.json             # 前端格式化规则
├── eslint.config.js             # 前端 lint(flat config)
├── package.json
├── pnpm-lock.yaml
├── tsconfig.json
├── vite.config.ts
└── README.md
```

---

## 4. 执行步骤

> 顺序执行。每步完成后再进入下一步,便于在出错时定位问题。

### 步骤 1:初始化 Tauri + Vue 项目

```bash
cd C:/Users/zyd/ZCodeProject
pnpm create tauri-app
```

交互提示按如下选择(脚手架版本可能略有差异,选最接近的):

| 提示 | 选择 |
| --- | --- |
| Project name | `file-toolkit`(或保持项目根名一致) |
| Identifier | `com.filetoolkit.app` |
| Frontend language | **TypeScript / JavaScript** → TypeScript |
| Package manager | **pnpm** |
| UI template | **Vue** |
| UI flavor | **TypeScript** |
| Tauri version | **Tauri 2** |

> 📌 这一步会把 `docs/` 之外的文件生成在工作目录。**注意:不要让脚手架覆盖 `docs/`。** 若脚手架生成在子目录,需把内容上移到项目根。

### 步骤 2:调整目录到我们的约定

脚手架默认前端结构较简,我们补齐:

```bash
# 前端目录
mkdir -p src/pages src/pipeline src/store src/components src/composables src/assets

# 后端模块骨架
mkdir -p src-tauri/src/commands src-tauri/src/pipeline src-tauri/src/worker src-tauri/src/common
```

然后在 `src-tauri/src/` 下用 `mod.rs` 声明各模块(见 §5 代码模板)。

### 步骤 3:实现"端到端调通"示例

**目标**:前端一个按钮 → 调 Rust 的 `greet` 命令 → 展示返回。

后端 `src-tauri/src/commands/mod.rs`:

```rust
pub mod image;      // M1 填充
pub mod pdf;        // M1 填充
pub mod video;      // M2 填充
pub mod rename;     // M1 填充
pub mod dedup;      // M1 填充

/// 调通用命令,M0 用于验证前后端通信。
#[tauri::command]
pub fn greet(name: &str) -> String {
    format!("来自 Rust 后端的问候:你好,{}!FileToolkit 已就绪。", name)
}
```

前端 `src/pages/Home.vue`(调通示例):

```vue
<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";

const name = ref("世界");
const reply = ref("");

async function onGreet() {
  reply.value = await invoke<string>("greet", { name: name.value });
}
</script>

<template>
  <main class="home">
    <h1>FileToolkit</h1>
    <p>一站式本地文件批量处理工具</p>
    <input v-model="name" placeholder="输入名字" />
    <button @click="onGreet">调用 Rust 后端</button>
    <p v-if="reply" class="reply">{{ reply }}</p>
  </main>
</template>
```

在 `src-tauri/src/lib.rs` 注册命令:

```rust
mod commands;
mod common;
mod pipeline;
mod worker;

#[cfg_attr(mobile, tauri::mobile_main)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![commands::greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

### 步骤 4:接入代码质量工具链

**前端(Prettier + ESLint)**:

```bash
pnpm add -D prettier eslint @eslint/js typescript-eslint eslint-plugin-vue vue-eslint-parser
```

`.prettierrc.json`:

```json
{
  "semi": true,
  "singleQuote": false,
  "printWidth": 100,
  "tabWidth": 2,
  "trailingComma": "all"
}
```

`eslint.config.js`(flat config,ESLint 9):

```js
import js from "@eslint/js";
import tseslint from "typescript-eslint";
import pluginVue from "eslint-plugin-vue";

export default [
  js.configs.recommended,
  ...tseslint.configs.recommended,
  ...pluginVue.configs["flat/recommended"],
  {
    rules: {
      "vue/multi-word-component-names": "off",
    },
  },
];
```

在 `package.json` 加 scripts:

```json
{
  "scripts": {
    "dev": "vite",
    "build": "vue-tsc --noEmit && vite build",
    "preview": "vite preview",
    "tauri": "tauri",
    "lint": "eslint . --ext .ts,.vue",
    "format": "prettier --write \"src/**/*.{ts,vue,json,css}\"",
    "rust:fmt": "cargo fmt --manifest-path src-tauri/Cargo.toml",
    "rust:clippy": "cargo clippy --manifest-path src-tauri/Cargo.toml -- -D warnings",
    "rust:test": "cargo test --manifest-path src-tauri/Cargo.toml"
  }
}
```

**Rust(clippy + fmt 基线)**:

`src-tauri/Cargo.toml` 预留后续依赖(本阶段只加 `serde`,调通需要):

```toml
[dependencies]
tauri = { version = "2", features = [] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"

[features]
default = ["custom-protocol"]
custom-protocol = ["tauri/custom-protocol"]
```

### 步骤 5:验证与验收

逐项检查(见 §5 验收清单)。

---

## 5. 验收清单(Acceptance Criteria)

M0 视为完成,需**全部**满足:

- [ ] `node --version`、`pnpm --version`、`rustc --version`、`cargo --version` 均正常
- [ ] `pnpm install` 无致命错误
- [ ] `pnpm tauri dev` 能启动并弹出桌面窗口
- [ ] 窗口中点击"调用 Rust 后端"按钮,正确显示 Rust 返回的问候语
- [ ] `pnpm lint` 通过(无 error)
- [ ] `pnpm format --check` 通过(格式一致)
- [ ] `cargo clippy --manifest-path src-tauri/Cargo.toml -- -D warnings` 通过
- [ ] `cargo fmt --manifest-path src-tauri/Cargo.toml --check` 通过
- [ ] 目录结构与 §3 一致(`commands/`、`pipeline/`、`worker/`、`common/` 骨架存在)
- [ ] `docs/` 完好,未被脚手架覆盖
- [ ] 已初始化 git 仓库并完成首次提交(可选,建议)

---

## 6. M0 完成后的下一步

M0 验收通过后,即进入 **M1(MVP 功能开发)**。M1 将按 PRD §3.1 的标记,优先实现:

1. 图片批量压缩 / 格式转换(`commands/image.rs`)
2. PDF 合并 / 拆分(`commands/pdf.rs`)
3. 文件批量重命名(`commands/rename.rs`)
4. 重复文件查重(`commands/dedup.rs`)

每个功能的设计文档在启动前补齐,放入 `docs/design/`。

---

## 7. 常见坑与注意事项(Windows 特定)

1. **MSVC 链接器缺失** → 表现为 `error: linker 'link.exe' not found`。确认 Build Tools 的 C++ 工作负载已勾选,并重启终端让环境变量生效。
2. **WebView2 未安装** → 表现为窗口空白或启动失败。从微软官网下载 "Evergreen Bootstrapper" 安装。
3. **脚手架覆盖 docs/** → 初始化前备份 `docs/`,初始化后核对;若脚手架坚持在子目录生成,把生成内容上移。
4. **pnpm 与 Tauri 的 node_modules** → 确保 `tauri.conf.json` 的 `frontendDist` 与 `beforeBuildCommand` 指向正确的 vite 产物路径。
5. **路径含中文/空格** → 项目已位于 `C:/Users/zyd/ZCodeProject`,无中文,但 `cargo` 在路径含空格时偶有问题,避免移动到含空格目录。
6. **杀毒软件拦截编译** → 部分 Windows 安全策略会拦截首次 `cargo build`,可将项目目录加入排除项。

---

## 8. 变更记录

| 日期       | 版本 | 变更内容                | 变更人 |
| ---------- | ---- | ----------------------- | ------ |
| 2026-07-02 | v1.0 | 初始创建 M0 骨架设计文档 | —      |

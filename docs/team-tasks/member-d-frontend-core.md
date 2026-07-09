# 成员 D 任务卡 — 前端核心架构

> **你的角色**:Vue 前端核心。你写的是所有前端页面共享的基础设施——布局、通用组件、流水线编辑器、国际化、主题。
> 你的产出直接决定整个应用的 UI 一致性和用户体验,与 A 并列为最先动工的两人。
> **代号**:FrontendCore

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
| D-1 | 前端基础设施 | M1 | 无(与 A-1 同时开工) | 与 A-1/A-2 并行 |
| D-2 | 统一通用组件 | M1 | D-1 | B/C 写功能时穿插 |
| D-3 | 流水线可视化编辑器 | M3 | A-6 | — |
| D-4 | 流水线模板系统 | M3 | D-3 | — |
| D-5 | 磁盘占用可视化 | M4 | 无 | 与 B-3/C-4 并行 |
| D-6 | 国际化 i18n | M5 | 功能稳定后 | 与 D-7 并行 |
| D-7 | 主题与视觉打磨 | M5 | 功能稳定后 | 与 D-6 并行 |

---

## 三、每步详解

### D-1 — 前端基础设施(Step 3)

> ⚠️ 你与成员 A 同时开工,你写前端基建,A 写后端基建。你不需要等 A——先用 `greet` 命令(已有)验证你的代码能通。

**做什么**:

1. **装依赖**:
   ```bash
   pnpm add pinia vue-router element-plus @element-plus/icons-vue
   pnpm add -D prettier eslint @eslint/js typescript-eslint eslint-plugin-vue vue-eslint-parser
   ```

2. **路由配置**(`src/router/index.ts`):
   ```typescript
   import { createRouter, createWebHashHistory } from "vue-router";

   const routes = [
     { path: "/", name: "home", component: () => import("../pages/HomePage.vue") },
     { path: "/image", name: "image", component: () => import("../pages/ImagePage.vue") },
     { path: "/pdf", name: "pdf", component: () => import("../pages/PdfPage.vue") },
     { path: "/rename", name: "rename", component: () => import("../pages/RenamePage.vue") },
     { path: "/dedup", name: "dedup", component: () => import("../pages/DedupPage.vue") },
     // 后续视频/音频等页面在此追加
   ];
   ```
   - 功能页先写占位组件(一个 `<div>ImagePage - 待开发</div>`),后续成员 B/C/E 填充

3. **Pinia Store**(`src/store/task.ts`):
   ```typescript
   import { defineStore } from "pinia";
   import { ref } from "vue";

   export const useTaskStore = defineStore("task", () => {
     const currentTask = ref<{id: string; progress: number; status: string} | null>(null);
     const taskHistory = ref<TaskRecord[]>([]);

     function startTask(id: string) { ... }
     function updateProgress(current: number, total: number) { ... }
     function completeTask(id: string) { ... }
     function cancelTask() { ... }

     return { currentTask, taskHistory, startTask, updateProgress, completeTask, cancelTask };
   });
   ```

4. **AppLayout 组件**(`src/components/AppLayout.vue`):
   ```
   ┌──────────┬──────────────────────────┐
   │ 侧边栏    │                          │
   │  🏠 首页  │      <router-view />     │
   │  🖼 图片  │      (页面内容区)         │
   │  📄 PDF   │                          │
   │  ✏️ 重命名│                          │
   │  🔍 查重  │                          │
   └──────────┴──────────────────────────┘
   ```
   - 侧边栏用 Element Plus 的 `el-menu`,支持折叠

5. **改造 `main.ts`**:
   ```typescript
   import { createApp } from "vue";
   import { createPinia } from "pinia";
   import ElementPlus from "element-plus";
   import "element-plus/dist/index.css";
   import App from "./App.vue";
   import router from "./router";

   const app = createApp(App);
   app.use(createPinia());
   app.use(router);
   app.use(ElementPlus);
   app.mount("#app");
   ```

6. **改造 `App.vue`**:保留 greet 示例但移到一个独立 `HomePage.vue`,根组件改为 `<AppLayout><router-view /></AppLayout>`

7. **ESLint + Prettier 配置**:

   `.prettierrc.json`:
   ```json
   { "semi": true, "singleQuote": false, "printWidth": 100, "tabWidth": 2, "trailingComma": "all" }
   ```

   `eslint.config.js`:
   ```js
   import js from "@eslint/js";
   import tseslint from "typescript-eslint";
   import pluginVue from "eslint-plugin-vue";

   export default [
     js.configs.recommended,
     ...tseslint.configs.recommended,
     ...pluginVue.configs["flat/recommended"],
     { rules: { "vue/multi-word-component-names": "off" } },
   ];
   ```

   `package.json` 加 scripts:
   ```json
   "lint": "eslint . --ext .ts,.vue",
   "format": "prettier --write \"src/**/*.{ts,vue,json,css}\""
   ```

**验收**:
- [ ] 应用打开有侧边栏+主区布局,可切换 4 个占位页面
- [ ] `pnpm lint` 通过
- [ ] `pnpm format` 无改动(格式一致)

---

### D-2 — 统一通用组件(Step 8)

> ⚠️ 这步不需要单独花时间,你在 B/C 写功能页的过程中,发现重复 UI 就抽成组件。

**要做的组件**:

1. **`components/FileDropZone.vue`**:
   - Props:`accept`(文件类型限制,如 "image/*" / ".pdf")
   - 支持拖拽 + 点击选择(多文件)
   - Emit:`@files-selected`(File[] 或路径字符串数组)
   - 视觉:虚线边框区域,拖入时高亮

2. **`components/TaskProgress.vue`**:
   - Props:`progress`(0-100), `status`("running"|"done"|"error"), `message`
   - Emit:`@cancel`
   - 元素:进度条(el-progress) + 取消按钮 + 状态文字

3. **`components/ResultList.vue`**:
   - Props:`items`({name, originalSize, newSize, status, error?}[])
   - 每行显示:文件名 + 状态图标(✅/❌) + 体积对比 + 错误信息(可展开)

4. **`composables/useBatchTask.ts`**:
   ```typescript
   export function useBatchTask(taskName: string) {
     const store = useTaskStore();
     async function run<T>(invokeCmd: string, args: Record<string, unknown>): Promise<T> {
       // 1. startTask
       // 2. listen("task-progress", updateProgress)
       // 3. await invoke(...)
       // 4. completeTask / handle error
     }
     return { run, cancel: store.cancelTask, progress, status };
   }
   ```

**验收**:成员 B/C 在他们的功能页中能用这些组件,行为一致。

---

### D-3 — 流水线可视化编辑器(Step 19)

> M3 阶段做,等成员 A 的流水线引擎(A-6)数据模型定义好。

**做什么**:

1. **技术选型**:推荐用 [Vue Flow](https://vueflow.dev/)(基于 React Flow 的 Vue 3 版本),节点式拖拽编排成熟方案。装:`pnpm add @vue-flow/core @vue-flow/background @vue-flow/controls`

2. **实现 `src/pipeline/PipelineEditor.vue`**:
   - 左侧:节点面板(可拖拽的节点类型列表:图片压缩/PDF合并/重命名/查重……)
   - 中间:画布区,Vue Flow 渲染节点和连线
   - 右侧:选中节点的参数编辑面板
   - 校验:实时检测是否有环(拓扑排序失败=有环)、必填参数是否齐全

3. **节点定义**:每个节点类型来自成员 A 的 `registry.rs`(A-7)。你和他提前约定 JSON schema:
   ```typescript
   interface NodeType {
     id: string; name: string; icon: string;
     params: { key: string; label: string; type: "string"|"number"|"boolean"|"select"; required: boolean; options?: string[] }[];
   }
   ```
   - 在前端维护一份节点类型列表(与后端 registry 一一对应)
   - 将来可以通过 API `get_node_types()` 从后端拉取,但前期硬编码更快

**验收**:拖拽加节点→连线→配参数→点运行;有环时阻止并标红。

---

### D-4 — 流水线模板系统(Step 20)

**做什么**:

1. 内置模板库(`src/pipeline/templates/`):
   - `photo-organize.json`:"图片压缩(WebP,1920px) → 重命名({date}-{index:3}) → 打包(zip)"
   - `contract-standardize.json`:"PDF 合并 → 压缩 → 加密"
   - 每个模板是一个 `Pipeline` JSON(与 A-6 的 `Pipeline` 结构一致)

2. **模板管理 UI**:
   - 模板列表(内置 + 用户保存的)
   - [另存为模板]按钮:导出当前画布为 JSON
   - [导入模板]按钮:选 JSON 文件加载到画布
   - [导出为 JSON] / [从 JSON 导入]

**验收**:一键加载模板,导出→导入正确还原。

---

### D-5 — 磁盘占用可视化(Step 27)

**做什么**:

1. 编写前端 `pages/DiskUsagePage.vue`:
   - 选目录 → 调后端命令(后端逻辑较简单,可自己写 `commands/diskusage.rs` 或委托成员 A/C)
   - 用 **ECharts** 的旭日图(sunburst)或树状图(treemap)展示目录体积
   - `pnpm add echarts vue-echarts`

2. 后端 `commands/diskusage.rs`:
   ```rust
   #[tauri::command]
   fn scan_directory(app: AppHandle, dir: String) -> Result<DirNode, String>
   ```
   - 递归遍历目录,返回 `DirNode { name, size, children }`
   - 大目录流式遍历(`walkdir` crate),emit 进度

**验收**:10GB+ 目录秒级扫描;旭日图层级清晰,鼠标悬停显示路径和大小。

---

### D-6 — 国际化 i18n(Step 28)

**做什么**:

1. `pnpm add vue-i18n`

2. 创建 `src/locales/zh-CN.json` 和 `src/locales/en.json`:
   ```json
   {
     "app.title": "FileToolkit",
     "nav.home": "首页",
     "nav.image": "图片处理",
     "image.compress": "压缩图片",
     "common.cancel": "取消",
     "common.progress": "进度"
   }
   ```

3. 在 `main.ts` 注册 i18n,逐个页面把硬编码文案换成 `$t("key")`

**验收**:切换语言后所有 UI 正确;新增语言只需加 json。

---

### D-7 — 主题与视觉打磨(Step 29)

**做什么**:

1. 用 Element Plus 的 CSS 变量覆盖,统一深/浅色主题
2. 项目图标(设计或找设计师)
3. 统一字体、间距、圆角、阴影等视觉规范

**验收**:深浅切换流畅;图标在任务栏清晰。

---

## 四、协作点

| 你产出什么 | 谁需要 | 时机 |
|---|---|---|
| 前端布局+路由 | B、C、E(所有写页面的人) | D-1 完成后 |
| `FileDropZone.vue`、`TaskProgress.vue`、`ResultList.vue` | B、C、E | D-2 逐步产出 |
| `useBatchTask.ts` | B、C、E | D-2 完成后 |
| 流水线编辑器 | A(联调引擎) | D-3 |
| 模板系统 | 全员(演示用) | D-4 |
| i18n + 主题 | 全员(最终产品) | D-6/D-7 |

---

## 五、验收总清单

- [ ] D-1:侧边栏布局+路由切换+lint 通过
- [ ] D-2:4 个通用组件被各功能页复用
- [ ] D-3:流水线画布,拖拽连线,参数配置,环检测
- [ ] D-4:模板一键加载+导出/导入
- [ ] D-5:旭日图磁盘可视化,10GB+ 秒级
- [ ] D-6:中英双语切换完整
- [ ] D-7:深浅主题+图标+视觉统一

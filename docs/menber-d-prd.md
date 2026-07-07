# PRD — 成员 D 专属产品需求文档(前端核心架构)

> 本文档定义成员 D(FrontendCore)负责范围内所有前端基础设施与共享功能的产品需求。
> 上游依据:[全项目 PRD](./PRD.md) §3 功能需求、§4 非功能需求、§5 技术架构。
> 本文聚焦"前端要做成什么样",不涉及后端实现细节。

| 项目     | 内容                  |
| -------- | --------------------- |
| 文档版本 | v1.0                  |
| 创建日期 | 2026-07-07            |
| 负责人   | 成员 D(FrontendCore) |
| 状态     | 草案 / 待评审         |

---

## 1. 背景与目标

### 1.1 背景

FileToolkit 计划实现图片、PDF、视频、音频、文件管理等十余项功能,每项都需要前端页面。若每个功能页"各写各的",必然导致:

- **UI 不一致**:不同页面的拖拽区、进度条、结果展示行为各异
- **代码重复**:每个页面都重复实现 `invoke + 事件监听 + 进度更新 + 错误处理`
- **维护困难**:改一处交互要在 N 个页面同步修改
- **上手成本高**:新成员(成员 E)接手功能页时缺少统一规范

成员 D 的职责正是消除这些问题——**先于所有功能页,沉淀一套统一的前端基础设施与通用组件**。

### 1.2 目标

为 FileToolkit 全部功能页面提供:

1. 一致的应用骨架(导航 + 布局 + 路由)
2. 统一的全局状态管理(任务生命周期)
3. 可复用的通用组件(拖拽区 / 进度条 / 结果列表)
4. 标准化的任务执行抽象(`useBatchTask`)
5. 流水线灵魂功能的可视化编辑器
6. 国际化与主题支持

### 1.3 核心价值主张

- 🧩 **积木化**:B/C/E 拿到积木即可拼装功能页,无需重复造轮子
- 🎯 **一致性**:全应用交互行为统一,用户体验专业
- 🌍 **可扩展**:新增语言/主题/功能页只需遵循既定规范
- 🔗 **差异化**:流水线可视化编辑器是项目区别于单一工具的核心 UI

---

## 2. 范围

### 2.1 本文档覆盖(成员 D 负责)

| 模块 | 内容 |
| ---- | ---- |
| 应用骨架 | 路由配置、布局组件、入口改造 |
| 状态管理 | Pinia task store |
| 通用组件 | FileDropZone / TaskProgress / ResultList |
| 任务抽象 | useBatchTask composable |
| 流水线编辑器 | 节点编排画布 + 参数表单 + 校验 |
| 模板系统 | 内置模板 + 导入导出 |
| 磁盘可视化 | 旭日图页面 |
| 国际化 | vue-i18n 中英双语 |
| 主题 | 深浅色 + 图标 + 视觉规范 |
| 工程化 | ESLint / Prettier / TypeScript 规范 |

### 2.2 本文档不覆盖(其他成员负责)

- 各功能页的业务逻辑实现(B/C/E)
- 后端命令的 Rust 实现(A/B/C)
- CI/CD、打包、发布(E)
- 单元测试用例编写(E-5 主导,D 配合)

---

## 3. 功能需求

### 3.1 应用骨架(D-1)

#### FR-D1.1 路由系统

- 应用采用 `vue-router`,`createWebHashHistory`(适配 Tauri)
- 内置路由:首页 / 图片 / PDF / 重命名 / 查重,后续可追加视频/音频/磁盘/流水线
- 功能页未实现时显示占位组件("XxxPage - 待开发")
- 路由切换时保持侧边栏选中态与当前页一致

#### FR-D1.2 全局布局(AppLayout)

```
┌──────────┬────────────────────────────┐
│ 侧边栏    │                            │
│  🏠 首页  │                            │
│  🖼 图片  │      <router-view />       │
│  📄 PDF   │      (页面内容区)          │
│  ✏️ 重命名│                            │
│  🔍 查重  │                            │
└──────────┴────────────────────────────┘
```

- 侧边栏使用 Element Plus `el-menu`,支持折叠/展开
- 侧边栏宽度可调,折叠状态持久化
- 内容区自适应剩余空间,支持滚动
- 窗口标题随当前路由更新

#### FR-D1.3 状态管理(Pinia task store)

- 维护 `currentTask`:`{ id, progress, status } | null`
- 维护 `taskHistory`:最近 N 条任务记录
- 提供 action:`startTask` / `updateProgress` / `completeTask` / `cancelTask`
- store 状态可在 Vue DevTools 中观察

#### FR-D1.4 入口改造

- `main.ts` 注入 Pinia、Router、ElementPlus 及其样式
- `App.vue` 改为 `<AppLayout />` 根组件
- 现有 greet 示例迁入 `HomePage.vue`

### 3.2 通用组件(D-2)

#### FR-D2.1 FileDropZone(文件导入区)

| 维度 | 规格 |
| ---- | ---- |
| Props | `accept: string`(如 `"image/*"` / `".pdf"`),`multiple: boolean`(默认 true) |
| Emit | `@files-selected(payload: string[] \| File[])` |
| 交互 | 拖拽文件高亮虚线边框 + 点击触发系统文件选择 |
| 视觉 | 拖入时边框变色 + 提示文案"松开以添加" |
| 约束 | `accept` 限制可选文件类型;超限提示 |

#### FR-D2.2 TaskProgress(任务进度)

| 维度 | 规格 |
| ---- | ---- |
| Props | `progress: number`(0-100),`status: "running" \| "done" \| "error"`,`message?: string` |
| Emit | `@cancel()` |
| 元素 | `el-progress` 进度条 + 取消按钮 + 状态文字 |
| 行为 | 状态为 `done` 时进度条满绿;`error` 时变红并显示错误信息 |

#### FR-D2.3 ResultList(结果列表)

| 维度 | 规格 |
| ---- | ---- |
| Props | `items: { name, originalSize, newSize, status, error? }[]` |
| 每行 | 文件名 + 状态图标(✅/❌)+ 体积对比(原 → 新,显示压缩率) |
| 交互 | 错误项可展开查看详细错误信息 |
| 汇总 | 顶部显示总数 / 成功 / 失败 / 总节省体积 |

#### FR-D2.4 useBatchTask(任务执行抽象)

- 签名:`useBatchTask(taskName: string)` 返回 `{ run, cancel, progress, status }`
- `run<T>(invokeCmd, args)` 内部:启动任务 → 监听 `task-progress` → `await invoke` → 完成/失败处理 → 自动解绑监听器
- 组件卸载时自动解绑事件监听,防止内存泄漏
- `cancel()` 调用后端取消并停止进度更新

### 3.3 流水线可视化编辑器(D-3)

#### FR-D3.1 画布与节点

- 基于 Vue Flow 实现节点式画布
- 三栏布局:左侧节点面板 / 中间画布 / 右侧参数面板
- 节点可从左侧拖入画布
- 节点可拖动定位、可删除、可选中

#### FR-D3.2 连线

- 节点间可拉线连接(输出端口 → 输入端口)
- 连线可删除(点击连线或右键菜单)
- 一个输出可连多个输入(扇出)

#### FR-D3.3 参数配置

- 选中节点后,右侧面板根据节点 schema 动态生成表单
- 字段类型:string / number / boolean / select
- 必填字段缺失时表单标红
- 参数变更实时反映到画布数据

#### FR-D3.4 校验

- **环检测**:若拓扑排序失败(存在环),阻止运行并标红相关节点/连线
- **必填校验**:必填参数缺失时高亮节点
- **连通性校验**:孤立节点(无连入也无连出)给出提示

#### FR-D3.5 序列化

- 画布状态可序列化为 `Pipeline` JSON(与后端 A-6 结构一致):
  ```json
  { "nodes": [{ "id", "node_type", "params" }], "edges": [{ "from", "to" }] }
  ```
- 可从 JSON 反序列化还原画布

### 3.4 流水线模板系统(D-4)

#### FR-D4.1 内置模板

- 至少提供 2 个模板:
  - **照片整理归档**:图片压缩(WebP,1920px)→ 重命名(`{date}-{index:3}`)→ 打包(zip)
  - **合同 PDF 标准化**:PDF 合并 → 压缩 → 加密
- 模板以 JSON 文件存放在 `src/assets/templates/`

#### FR-D4.2 模板管理 UI

- 模板列表:展示内置 + 用户保存的模板
- 操作:[加载到画布] / [另存为模板] / [导出 JSON] / [从 JSON 导入]
- 用户保存的模板持久化到 localStorage

#### FR-D4.3 导入导出

- 导出:当前画布 → 下载 JSON 文件
- 导入:选 JSON 文件 → 校验结构 → 加载到画布
- 导出→导入必须完整还原节点、连线、参数

### 3.5 磁盘占用可视化(D-5)

#### FR-D5.1 扫描入口

- 选目录按钮 → 调后端 `scan_directory` → 流式返回 `DirNode` 树
- 扫描中显示进度,可取消

#### FR-D5.2 可视化

- 用 ECharts 旭日图(sunburst)或树状图(treemap)展示
- 层级:盘符 → 目录 → 子目录 → 文件
- 鼠标悬停显示完整路径和大小
- 点击某层可下钻/返回

### 3.6 国际化(D-6)

#### FR-D6.1 多语言支持

- 接入 vue-i18n
- 提供简体中文(zh-CN)+ 英文(en)双语
- 所有 UI 文案(菜单/按钮/提示/错误)外置为 `$t("key")`

#### FR-D6.2 语言切换

- 语言切换入口(设置页或顶栏)
- 切换后所有 UI 实时更新
- 语言偏好持久化到 localStorage

#### FR-D6.3 扩展性

- 新增语言只需新增一个 json 文件,无需改代码

### 3.7 主题(D-7)

#### FR-D7.1 深浅色主题

- 统一 CSS 变量(主色/中性色/间距/圆角/阴影)
- Element Plus 主题覆盖
- 深浅切换流畅,无白屏闪烁

#### FR-D7.2 主题模式

- 三种模式:浅色 / 深色 / 跟随系统
- 模式选择持久化到 localStorage

#### FR-D7.3 应用图标

- 提供各尺寸图标(png 16/32/128/256/512 + ico + icns)
- 图标在 Win/Mac 任务栏清晰可辨

---

## 4. 非功能需求

### 4.1 一致性

- 所有功能页必须复用 D 提供的通用组件,不得自行实现重复 UI
- 视觉风格(字体/间距/配色/圆角)统一遵循 D-7 的视觉规范

### 4.2 类型安全

- 前端必须使用 TypeScript,禁用纯 JS
- 组件 Props/Emit、store、composable 均需显式类型标注
- `pnpm build`(含 `vue-tsc --noEmit`)零类型错误

### 4.3 代码质量

- `pnpm lint` 零 error
- `pnpm format` 后无格式差异
- 组件单一职责,可复用组件放 `components/`,页面放 `pages/`,逻辑放 `composables/`

### 4.4 性能

- 路由懒加载(`() => import(...)`),首屏不加载所有功能页
- 大列表(如结果列表 1000 项)考虑虚拟滚动
- 画布节点数量超过 100 时仍流畅(无明显卡顿)

### 4.5 可访问性(a11y)

- 键盘可达:核心操作(导航切换、按钮)支持 Tab + Enter
- 焦点可见(focus-visible 样式)
- 图标按钮提供 `aria-label`

### 4.6 国际化就绪

- 所有文案通过 `$t()` 引用,不硬编码中文(开发期占位除外)
- 日期/数字格式遵循 locale

---

## 5. 技术选型

| 用途 | 选型 | 理由 |
| ---- | ---- | ---- |
| 框架 | Vue 3(组合式 API) | 团队已有基础,PRD 已定 |
| 语言 | TypeScript | 强制,禁用纯 JS |
| 状态管理 | Pinia | Vue 3 官方推荐,TypeScript 友好 |
| 路由 | Vue Router 4 | 官方方案 |
| UI 库 | Element Plus | 组件丰富,中文生态好 |
| 节点编辑器 | Vue Flow | Vue 3 原生,基于成熟的 React Flow |
| 图表 | ECharts + vue-echarts | 功能全,旭日图/树图开箱即用 |
| 国际化 | vue-i18n | 官方方案 |
| Lint | ESLint + typescript-eslint + eslint-plugin-vue | 类型 + 模板双查 |
| 格式化 | Prettier | 团队统一风格 |
| 构建 | Vite | 已配置,Tauri 默认 |

---

## 6. 接口约定(与后端/其他成员)

### 6.1 事件协议(监听后端)

| 事件名 | 载荷 | 来源 | 用途 |
| ------ | ---- | ---- | ---- |
| `task-progress` | `{ current: number, total: number, message: string }` | A-2 worker | `useBatchTask` 更新进度 |
| `pipeline-progress` | `{ node_id, current, total }` | A-6 引擎 | 流水线执行进度(预留) |

### 6.2 命令调用(前端 invoke)

成员 D 不直接调用业务命令,但 `useBatchTask` 封装了通用调用模式:

```typescript
const { run, cancel, progress } = useBatchTask("compress_images");
const results = await run("compress_images", { files, quality: 80, ... });
```

### 6.3 流水线数据契约(与 A-6/A-7)

前端画布序列化的 JSON 必须与后端 `Pipeline` 结构一致:

```typescript
interface Pipeline {
  nodes: { id: string; node_type: string; params: Record<string, unknown> }[];
  edges: { from: string; to: string }[];
}
```

节点 schema(来自 A-7 registry,前端前期可硬编码):

```typescript
interface NodeType {
  id: string;
  name: string;
  icon: string;
  params: {
    key: string;
    label: string;
    type: "string" | "number" | "boolean" | "select";
    required: boolean;
    options?: string[];
  }[];
}
```

---

## 7. 里程碑与交付节奏

| 阶段 | 交付内容 | 对应项目里程碑 |
| ---- | -------- | -------------- |
| D-1 | 应用骨架 + Lint | M1 |
| D-2 | 4 通用组件 + useBatchTask | M1 |
| D-3 | 流水线编辑器 | M3 |
| D-4 | 模板系统 | M3 |
| D-5 | 磁盘可视化 | M4 |
| D-6 | 国际化 | M5 |
| D-7 | 主题与视觉 | M5 |

详细时间线与依赖见 [`member-d-roadmap.md`](./member-d-roadmap.md)。

---

## 8. 待确认事项(Open Questions)

1. **UI 库最终选型**:Element Plus 还是 Naive UI?(任务卡默认 Element Plus,全项目 ROADMAP Step 3 写的是"Element Plus / Naive UI"二选一)—— **建议尽快定**。
2. **流水线画布库**:Vue Flow 还是自研?Vue Flow 引入体积约几百 KB,可接受。
3. **磁盘可视化的后端**:D 自己写 `commands/diskusage.rs`,还是委托成员 A/C?
4. **品牌视觉**:Logo、主色调方向?(影响 D-7,需与项目负责人确认)
5. **侧边栏功能项顺序**:按使用频率还是按功能分类?

---

## 9. 变更记录

| 日期       | 版本 | 变更内容                       | 变更人 |
| ---------- | ---- | ------------------------------ | ------ |
| 2026-07-07 | v1.0 | 初始创建,定义 D-1~D-7 全需求 | 成员 D |
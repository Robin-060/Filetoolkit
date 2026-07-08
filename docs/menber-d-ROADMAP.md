# ROADMAP — 成员 D 专属实施路线图(前端核心架构)

> 本文档是 [FileToolkit 全项目 ROADMAP](./ROADMAP.md) 在成员 D 视角下的细化版。
> 只覆盖成员 D(代号 FrontendCore)负责的范围,把横跨 M1–M5 的 7 个步骤拆成可独立交付、可验收的工作单元。

| 项目     | 内容                                       |
| -------- | ------------------------------------------ |
| 文档版本 | v1.0                                       |
| 创建日期 | 2026-07-07                                 |
| 负责人   | 成员 D(FrontendCore)                      |
| 总步骤数 | **7 步**(D-1 ~ D-7)                       |
| 当前阶段 | D-1 ⏳ 待启动(前端基础设施尚未落地)       |

---

## 一、角色定位

**你是全员 UI 的基石提供者。** 你产出的不是某个功能页,而是所有功能页赖以存在的东西:

- 应用骨架(路由 / 布局 / 状态管理)
- 通用组件(拖拽区 / 进度条 / 结果列表)
- 任务执行抽象(`useBatchTask`,封装 invoke + 事件监听)
- 流水线灵魂功能的可视化编辑器
- 国际化与主题(最终产品的"脸")

B / C / E 三个写功能页的人,用的全是你给的积木。

> 📌 与成员 A 并列为**最先动工**的两人:A 写后端地基,你写前端地基,互不阻塞。

> ⚠️ **实测代码库现状**(2026-07-07):`main.ts` 仅有 `createApp(App).mount("#app")`,`package.json` 未含 pinia/vue-router/element-plus/eslint/prettier,`App.vue` 仍是 M0 的 greet 示例。**即任务卡中标 ✅ 的 D-1 实际尚未开始**,这是当前最高优先级。

---

## 二、里程碑总览(成员 D 视角)

| D 阶段  | 名称               | 对应项目里程碑 | 步骤    | 状态 | 目标                              |
| ------- | ------------------ | -------------- | ------- | :--: | --------------------------------- |
| **D-1** | 前端基础设施       | M1             | Step 3  |  ⏳  | 路由+布局+状态+Lint,骨架可运行    |
| **D-2** | 统一通用组件       | M1             | Step 8  |  ⏸  | 4 组件 + 1 composable,全员复用    |
| **D-3** | 流水线可视化编辑器 | M3             | Step 19 |  ⏸  | 节点式拖拽编排,差异化能力的"脸"  |
| **D-4** | 流水线模板系统     | M3             | Step 20 |  ⏸  | 内置模板 + 导入导出               |
| **D-5** | 磁盘占用可视化     | M4             | Step 27 |  ⏸  | ECharts 旭日图,10GB+ 秒级         |
| **D-6** | 国际化 i18n        | M5             | Step 28 |  ⏸  | 中英双语,文案外置可扩展           |
| **D-7** | 主题与视觉打磨     | M5             | Step 29 |  ⏸  | 深浅主题 + 图标 + 视觉规范         |

---

## 三、阶段详细步骤

### ◆ D-1 — 前端基础设施(Step 3)⏳ 待启动

**目标**:搭出可运行的应用骨架——侧边栏导航 + 主区路由切换 + 全局任务状态 + Lint 工具链。

**产出物**:

| 文件                                          | 内容                                                                      |
| --------------------------------------------- | ------------------------------------------------------------------------- |
| `package.json`                                | 新增 deps:pinia、vue-router、element-plus、@element-plus/icons-vue;devDeps:prettier、eslint、typescript-eslint、eslint-plugin-vue、vue-eslint-parser |
| `src/main.ts`                                 | 注入 createPinia + router + ElementPlus                                   |
| `src/router/index.ts`                         | 5 条路由(home/image/pdf/rename/dedup),功能页先用占位组件                |
| `src/store/task.ts`                           | Pinia 任务状态(currentTask / taskHistory + start/update/complete/cancel) |
| `src/components/AppLayout.vue`                | 侧边栏 `el-menu` + `<router-view />` 主区                                 |
| `src/pages/HomePage.vue`                      | 迁入现有 greet 示例                                                       |
| `src/pages/{Image,Pdf,Rename,Dedup}Page.vue`  | 占位组件(`<div>XxxPage - 待开发</div>`)                                   |
| `src/App.vue`                                 | 改为 `<AppLayout />` 包裹                                                 |
| `.prettierrc.json` / `eslint.config.js`       | 格式与静态检查配置                                                        |

**验收标准**:

- [ ] `pnpm install` 无报错
- [ ] `pnpm tauri dev` 启动后:侧边栏可折叠,点击切换 5 个页面(4 占位 + 首页 greet)
- [ ] `pnpm lint` 通过,零 error
- [ ] `pnpm format` 后 `git diff` 无变化(格式已统一)
- [ ] Pinia store 可在 Vue DevTools 中看到 `task` 模块

**依赖**:无(与 A-1/A-2 完全并行)
**可并行**:A 在写后端基建,你写前端基建,**互不阻塞**
**建议工期**:1.5 天

---

### ◆ D-2 — 统一通用组件(Step 8)⏸

> 不需要单独留大块时间。在 B/C 写功能页的过程中,一旦发现重复 UI 就抽成组件。建议在 D-1 完成后、M1 四大功能开发期间**穿插进行**。

**产出物**:

| 文件                              | 职责                                                                          |
| --------------------------------- | ----------------------------------------------------------------------------- |
| `src/components/FileDropZone.vue` | 拖拽 + 点击多选;Props:`accept`;Emit:`@files-selected`                       |
| `src/components/TaskProgress.vue`  | 进度条 + 取消按钮 + 状态文字;Props:`progress/status/message`;Emit:`@cancel` |
| `src/components/ResultList.vue`    | 每行:文件名 + ✅/❌ + 体积对比 + 可展开错误                                  |
| `src/composables/useBatchTask.ts`  | 封装 `invoke + listen("task-progress") + start/complete` 全流程               |

**验收标准**:

- [ ] 成员 B 的 `ImagePage.vue`、成员 C 的 `RenamePage.vue` 均复用这一套组件
- [ ] 取消按钮点击后,后端在 1-2 秒内停止(依赖 A-2 的 `cancel_flag`)
- [ ] `useBatchTask("compress_images").run({...})` 一行调用即跑通完整生命周期

**依赖**:D-1 + A-2(`BatchRunner` 已 emit `task-progress` 事件)
**可并行**:与 B-1 / C-1 功能开发穿插
**建议工期**:2-3 天(穿插)

---

### ◆ D-3 — 流水线可视化编辑器(Step 19)⏸

> M3 阶段做。等成员 A 的流水线引擎(A-6)数据模型定义好。这是项目**灵魂功能**的用户界面,也是 D 工作中最有技术含量的一步。

**技术选型**:[Vue Flow](https://vueflow.dev/)

```bash
pnpm add @vue-flow/core @vue-flow/background @vue-flow/controls @vue-flow/minimap
```

**产出物**:

| 文件                              | 内容                                                       |
| --------------------------------- | ---------------------------------------------------------- |
| `src/pipeline/PipelineEditor.vue` | 主编辑器:左节点面板 + 中画布 + 右参数面板                  |
| `src/pipeline/NodePanel.vue`      | 可拖拽的节点类型列表                                       |
| `src/pipeline/NodeParamForm.vue`  | 选中节点后的参数表单(按 JSON Schema 动态渲染)            |
| `src/pipeline/nodes/*.vue`        | 各节点类型的自定义渲染组件                                 |
| `src/composables/usePipeline.ts`  | 画布状态管理(增删节点/连线/校验/序列化为 JSON)            |

**验收标准**:

- [ ] 从左侧面板拖入节点到画布
- [ ] 节点之间可拉线连接
- [ ] 点击节点,右侧显示参数表单
- [ ] **环检测**:形成环时标红并阻止运行
- [ ] 必填参数缺失时高亮提示
- [ ] 画布状态可序列化为 A-6 定义的 `Pipeline` JSON 结构

**依赖**:A-6(`Pipeline` 类型)+ A-7(节点 schema)—— **需提前与 A 约定 JSON schema**
**可并行**:A-7(后端注册表),只要 schema 约定好
**建议工期**:4-5 天

---

### ◆ D-4 — 流水线模板系统(Step 20)⏸

**目标**:内置常用模板,让用户"一键加载"典型流水线,降低使用门槛。

**产出物**:

| 文件                                              | 内容                                       |
| ------------------------------------------------- | ------------------------------------------ |
| `src/assets/templates/photo-organize.json`        | "图片压缩 → 重命名 → 打包"               |
| `src/assets/templates/contract-standardize.json`  | "PDF 合并 → 压缩 → 加密"                 |
| `src/pipeline/TemplateManager.vue`                | 模板列表 + [加载]/[另存为]/[导入]/[导出]  |
| `src/composables/useTemplates.ts`                 | 读写 localStorage + 加载内置模板          |

**验收标准**:

- [ ] 一键加载内置模板,画布正确渲染所有节点和连线
- [ ] 当前画布可"另存为模板",刷新后仍在列表中
- [ ] 导出 JSON → 清空画布 → 导入 JSON,完整还原
- [ ] 模板 JSON 结构与 A-6 的 `Pipeline` 一致

**依赖**:D-3
**建议工期**:1-2 天

---

### ◆ D-5 — 磁盘占用可视化(Step 27)⏸

**技术选型**:ECharts(`echarts` + `vue-echarts`)

```bash
pnpm add echarts vue-echarts
```

**产出物**:

| 文件                                    | 内容                                                       |
| --------------------------------------- | ---------------------------------------------------------- |
| `src/pages/DiskUsagePage.vue`           | 选目录按钮 + 旭日图 + 悬停 tooltip                         |
| `src/components/SunburstChart.vue`      | ECharts 旭日图封装组件                                     |
| `src/composables/useDiskScan.ts`        | 调 `scan_directory` 命令 + 监听进度                        |
| `src-tauri/src/commands/diskusage.rs`   | `scan_directory(app, dir) → DirNode`(自写或委托 A/C)     |

**验收标准**:

- [ ] 选 10GB+ 目录,秒级返回结构
- [ ] 旭日图层级清晰,鼠标悬停显示完整路径和大小
- [ ] 扫描中有进度,可取消

**依赖**:D-1 + 后端 `scan_directory` 命令
**可并行**:与 B-3 / C-4 / C-5 全部并行
**建议工期**:2 天

---

### ◆ D-6 — 国际化 i18n(Step 28)⏸

```bash
pnpm add vue-i18n
```

**产出物**:

| 文件                     | 内容                                |
| ------------------------ | ----------------------------------- |
| `src/locales/zh-CN.json` | 简体中文文案                        |
| `src/locales/en.json`    | 英文文案                            |
| `src/i18n/index.ts`      | i18n 实例配置                       |
| `src/main.ts`            | 注入 i18n 插件                      |
| 各 `*.vue`               | 硬编码字符串替换为 `$t("key")`      |

**验收标准**:

- [ ] 切换语言后,所有 UI 文案(菜单/按钮/提示/错误)正确切换
- [ ] 新增语言只需加一个 json 文件
- [ ] 语言偏好持久化到 localStorage

**依赖**:功能稳定后(否则文案反复变动)
**可并行**:D-7
**建议工期**:2 天

---

### ◆ D-7 — 主题与视觉打磨(Step 29)⏸

**产出物**:

| 文件 / 资源                        | 内容                                       |
| ---------------------------------- | ------------------------------------------ |
| `src/styles/theme.css`             | CSS 变量(主色/中性色/间距/圆角/阴影)      |
| `src/styles/element-overrides.css` | Element Plus 主题覆盖                      |
| `src/composables/useTheme.ts`      | 深浅切换 + 跟随系统 + 持久化               |
| `src-tauri/icons/*`                | 应用图标(各尺寸 png + ico + icns)        |
| `docs/design/visual-spec.md`       | 视觉规范文档                               |

**验收标准**:

- [ ] 深浅主题切换流畅,无闪烁
- [ ] 跟随系统主题模式可开关
- [ ] 图标在 Win/Mac 任务栏清晰可辨
- [ ] 所有 Element Plus 组件视觉统一

**依赖**:功能稳定后
**可并行**:D-6
**建议工期**:2-3 天

---

## 四、与他人的依赖关系图

```
            A-1(错误/类型)──────┐
                                  │
            A-2(worker)──────────┼──► B-1/C-1 功能页
                                  │         ▲
   ★ D-1(前端基建)────────────────┼─────────┘ 你给 B/C/E 的地基
                                  │
            A-6(流水线引擎)       │
                │                  │
                ▼                  │
   ★ D-3(可视化编辑器)◄──约定 schema── A-7(注册表)
                │
                ▼
   ★ D-4(模板系统)
                                  │
   ★ D-5(磁盘可视化)◄──scan_directory 命令(自写或委托)
                                  │
   ★ D-6(i18n)  ∥  D-7(主题)  ◄── 等功能稳定
```

★ = 你的工作

---

## 五、协作点清单

### 你给别人(你的产出是别人的依赖)

| 你的产出                                       | 谁需要                  | 交付时机    |
| ---------------------------------------------- | ----------------------- | ----------- |
| AppLayout + 路由                               | B / C / E(所有写页面的)| D-1 完成后  |
| `FileDropZone` / `TaskProgress` / `ResultList` | B / C / E               | D-2 逐步    |
| `useBatchTask.ts`                              | B / C / E               | D-2 完成后  |
| 流水线编辑器                                   | A(联调引擎)、全员(演示)| D-3         |
| 模板系统                                       | 全员(演示用)          | D-4         |
| i18n + 主题                                    | 全员(最终产品)        | D-6 / D-7   |

### 别人给你(你等谁的产出)

| 你需要的                  | 从谁获取        | 等待时机    |
| ------------------------- | --------------- | ----------- |
| `task-progress` 事件协议  | A(A-2 worker)  | D-2 开始前  |
| `Pipeline` JSON schema    | A(A-6 引擎)    | D-3 开始前  |
| 节点类型 registry schema  | A(A-7)          | D-3 开始前  |
| 各功能后端命令签名        | B / C           | 对接时      |

---

## 六、优先级与建议节奏

1. **立即做 D-1**(前端基础设施)。当前代码库最大缺口:后端已推进到 A-5,前端仍停在 M0 greet 示例。你不动,B/C/E 全部卡住。
2. **D-2 穿插进行**。不要闭门把 4 个组件全写完再交付——B/C 一开始写功能页就会催 `FileDropZone`。先给最简版本,再迭代。
3. **D-3/D-4 等 M3**。流水线是灵魂功能,但要等 M1 四个原子功能做扎实才有意义。这段时间可预研 Vue Flow。
4. **D-5/D-6/D-7 收尾做**。功能不稳定时做 i18n/主题等于反复返工。
5. **每个步骤完成后**跑 `pnpm lint` + `pnpm format`,commit message 用 `feat(d-x): xxx`。

---

## 七、验收总清单

- [ ] **D-1**:侧边栏布局 + 路由切换 + Pinia store + lint 通过
- [ ] **D-2**:4 组件 + `useBatchTask` 被 B/C/E 复用
- [ ] **D-3**:Vue Flow 画布,拖拽连线,参数配置,环检测
- [ ] **D-4**:模板一键加载 + 导出/导入 JSON
- [ ] **D-5**:旭日图磁盘可视化,10GB+ 秒级
- [ ] **D-6**:中英双语切换完整,文案外置
- [ ] **D-7**:深浅主题 + 图标 + 视觉统一

---

## 八、变更记录

| 日期       | 版本 | 变更内容                          | 变更人 |
| ---------- | ---- | --------------------------------- | ------ |
| 2026-07-07 | v1.0 | 初始创建,D-1~D-7 共 7 步路线图   | 成员 D |
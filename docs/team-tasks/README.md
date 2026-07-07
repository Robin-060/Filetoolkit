# 5 人团队任务分配总览

> 每个人打开自己的文件,**独立可执行**。文件里标明了每一步依赖谁、谁能跟你并行。

| 成员 | 代号 | 文件 | 角色 | 主要步骤 |
|:--:|------|------|------|------|
| **A** | Core | [`member-a-core.md`](./member-a-core.md) | 后端核心架构 | 错误类型→任务队列→ffmpeg→视频→流水线引擎 |
| **B** | ImagePDF | [`member-b-image-pdf.md`](./member-b-image-pdf.md) | 图片+PDF全栈 | 图片压缩全栈→PDF后端→Office→解压 |
| **C** | RenameDedup | [`member-c-rename-dedup.md`](./member-c-rename-dedup.md) | 重命名+查重+音频 | 重命名全栈→查重后端→音频→OCR→校验 |
| **D** | FrontendCore | [`member-d-frontend-core.md`](./member-d-frontend-core.md) | 前端核心架构 | 前端基建→通用组件→流水线编辑器→i18n→主题 |
| **E** | FrontendDevOps | [`member-e-frontend-devops.md`](./member-e-frontend-devops.md) | 前端功能页+DevOps | PDF/查重/视频前端→CI/CD→测试→打包→发布 |

## 启动顺序

```
第1天: A(A-1) + D(D-1) 同时开工
     ↓
第2-3天: A(A-2), D(D-2 开始)
     ↓
第4天起: B(B-1), C(C-1), E(E-1, E-4) 全员并行
     ↓
持续: 按各人文件顺序推进
```

## 关键协作节点

| 节点 | A 给 B/C | D 给全员 | B 给 E | C 给 E |
|---|---|---|---|---|
| 产出 | `AppError` `Task` 类型 + `worker` 框架 | 布局+路由+`FileDropZone` 等组件 | `merge_pdfs` 等命令接口 | `scan_duplicates` 等命令接口 |
| 谁等 | B、C 的 M1 功能 | B、C、E 的前端页面 | E 的 PDF 前端 | E 的查重前端 |

## 公共文档(全员都要看)

- [`docs/CONTRIBUTING.md`](../CONTRIBUTING.md) — 环境搭建 + 开发工作流 + FAQ
- [`docs/AGENTS.md`](../AGENTS.md) — 编码规范
- [`docs/PRD.md`](../PRD.md) — 功能需求详情

<script setup lang="ts">
import { useRouter } from "vue-router";
import { Picture, Document, EditPen, Search } from "@element-plus/icons-vue";

const router = useRouter();

interface ToolCard {
  path: string;
  icon: typeof Picture;
  title: string;
  description: string;
}

const tools: ToolCard[] = [
  {
    path: "/image",
    icon: Picture,
    title: "图片处理",
    description: "批量压缩、格式转换、调整尺寸",
  },
  {
    path: "/pdf",
    icon: Document,
    title: "PDF 工具",
    description: "合并、拆分、压缩 PDF 文件",
  },
  {
    path: "/rename",
    icon: EditPen,
    title: "批量重命名",
    description: "模板变量、实时预览、冲突检测",
  },
  {
    path: "/dedup",
    icon: Search,
    title: "重复查重",
    description: "智能扫描、分组展示、一键清理",
  },
];

function goTo(path: string) {
  router.push(path);
}
</script>

<template>
  <div class="home">
    <div class="hero">
      <h1>FileToolkit</h1>
      <p class="tagline">一站式本地文件批量处理工具</p>
      <p class="tagline-sub">本地优先 · 隐私安全 · 轻量高效</p>
    </div>

    <div class="tools-grid">
      <el-card
        v-for="tool in tools"
        :key="tool.path"
        class="tool-card"
        shadow="hover"
        @click="goTo(tool.path)"
      >
        <div class="tool-card-content">
          <el-icon :size="40" color="var(--el-color-primary)">
            <component :is="tool.icon" />
          </el-icon>
          <h3>{{ tool.title }}</h3>
          <p>{{ tool.description }}</p>
        </div>
      </el-card>
    </div>
  </div>
</template>

<style scoped>
.home {
  padding: 40px;
  max-width: 900px;
  margin: 0 auto;
}

.hero {
  text-align: center;
  margin-bottom: 48px;
}

h1 {
  margin: 0;
  font-size: 2.2em;
  color: var(--el-text-color-primary);
}

.tagline {
  color: var(--el-color-primary);
  font-weight: 500;
  margin-top: 0.5em;
  font-size: 1.1em;
}

.tagline-sub {
  color: var(--el-text-color-secondary);
  margin-top: 0.2em;
  font-size: 0.95em;
}

.tools-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(200px, 1fr));
  gap: 20px;
}

.tool-card {
  cursor: pointer;
  transition:
    transform 0.2s ease,
    border-color 0.2s ease;
}

.tool-card:hover {
  transform: translateY(-2px);
  border-color: var(--el-color-primary);
}

.tool-card-content {
  display: flex;
  flex-direction: column;
  align-items: center;
  text-align: center;
  padding: 8px 0;
}

.tool-card-content h3 {
  margin: 12px 0 6px;
  font-size: 1.05em;
  color: var(--el-text-color-primary);
}

.tool-card-content p {
  margin: 0;
  font-size: 0.85em;
  color: var(--el-text-color-secondary);
}
</style>

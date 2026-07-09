<script setup lang="ts">
import { ref } from "vue";
import { useRouter, useRoute } from "vue-router";
import { House, Picture, Document, EditPen, Search, Fold, Expand } from "@element-plus/icons-vue";

const router = useRouter();
const route = useRoute();
const isCollapsed = ref(false);

const navItems = [
  { path: "/", label: "首页", icon: House },
  { path: "/image", label: "图片处理", icon: Picture },
  { path: "/pdf", label: "PDF 工具", icon: Document },
  { path: "/rename", label: "批量重命名", icon: EditPen },
  { path: "/dedup", label: "重复查重", icon: Search },
];

function handleSelect(path: string) {
  router.push(path);
}
</script>

<template>
  <el-container class="app-layout">
    <!-- 侧边栏 -->
    <el-aside :width="isCollapsed ? '64px' : '200px'" class="app-aside">
      <div class="logo">
        <span v-if="!isCollapsed" class="logo-text">FileToolkit</span>
        <span v-else class="logo-text">FT</span>
      </div>
      <el-menu
        :default-active="route.path"
        :collapse="isCollapsed"
        :collapse-transition="false"
        class="app-menu"
        @select="handleSelect"
      >
        <el-menu-item v-for="item in navItems" :key="item.path" :index="item.path">
          <el-icon><component :is="item.icon" /></el-icon>
          <template #title>
            {{ item.label }}
          </template>
        </el-menu-item>
      </el-menu>
    </el-aside>

    <!-- 主区域 -->
    <el-container>
      <el-header class="app-header">
        <el-icon class="collapse-btn" :size="20" @click="isCollapsed = !isCollapsed">
          <component :is="isCollapsed ? Expand : Fold" />
        </el-icon>
      </el-header>
      <el-main class="app-main">
        <router-view />
      </el-main>
    </el-container>
  </el-container>
</template>

<style scoped>
.app-layout {
  height: 100vh;
}

.app-aside {
  border-right: 1px solid var(--el-border-color-light);
  overflow: hidden;
  transition: width 0.3s ease;
}

.logo {
  display: flex;
  align-items: center;
  justify-content: center;
  height: 60px;
  border-bottom: 1px solid var(--el-border-color-light);
}

.logo-text {
  font-size: 18px;
  font-weight: 700;
  color: var(--el-color-primary);
  white-space: nowrap;
}

.app-menu {
  border-right: none;
}

.app-header {
  display: flex;
  align-items: center;
  height: 48px;
  border-bottom: 1px solid var(--el-border-color-light);
  padding: 0 16px;
}

.collapse-btn {
  cursor: pointer;
  color: var(--el-text-color-regular);
}

.collapse-btn:hover {
  color: var(--el-color-primary);
}

.app-main {
  overflow-y: auto;
  background-color: var(--el-bg-color-page);
}
</style>

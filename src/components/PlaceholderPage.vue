<script setup lang="ts">
import type { Component } from "vue";

defineProps<{
  /** Element Plus 图标组件 */
  icon: Component;
  /** 功能标题 */
  title: string;
  /** 功能简介 */
  description: string;
}>();
</script>

<template>
  <div class="placeholder-page">
    <el-icon :size="64" color="var(--el-text-color-placeholder)">
      <component :is="icon" />
    </el-icon>
    <h2>{{ title }}</h2>
    <p>{{ description }}</p>
    <p class="hint">功能开发中...</p>
  </div>
</template>

<style scoped>
.placeholder-page {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  min-height: 60vh;
  text-align: center;
}

h2 {
  margin-top: 1em;
}

.hint {
  color: var(--el-text-color-secondary);
  font-size: 0.9em;
  margin-top: 0.5em;
}
</style>

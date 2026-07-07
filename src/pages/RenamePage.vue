<script setup lang="ts">
import { EditPen } from "@element-plus/icons-vue";
import PlaceholderPage from "../components/PlaceholderPage.vue";
</script>

<template>
  <PlaceholderPage
    :icon="EditPen"
    title="批量重命名"
    description="模板变量 / 实时预览 / 冲突检测"
  />
</template>

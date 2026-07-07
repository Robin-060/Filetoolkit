<script setup lang="ts">
import { computed, ref } from "vue";
import { CircleCheck, CircleClose, InfoFilled } from "@element-plus/icons-vue";

/**
 * 批量处理结果列表组件。
 * 展示每个文件的处理结果:文件名、体积对比、状态。
 * 顶部汇总:总数/成功/失败/总节省体积。
 *
 * Props:
 *   items — 结果数组,每项:
 *     name          文件名
 *     status        "success" | "error"
 *     originalSize  原始字节数(可选)
 *     newSize       处理后字节数(可选)
 *     error         错误信息(可选,status=error 时展示)
 */

export interface ResultItem {
  name: string;
  status: "success" | "error";
  originalSize?: number;
  newSize?: number;
  error?: string;
}

const props = defineProps<{
  items: ResultItem[];
}>();

const expandedErrors = ref<Record<number, boolean>>({});

const stats = computed(() => {
  const total = props.items.length;
  const success = props.items.filter((i) => i.status === "success").length;
  const failed = total - success;
  let totalSaved = 0;
  props.items.forEach((i) => {
    if (i.originalSize !== undefined && i.newSize !== undefined) {
      totalSaved += i.originalSize - i.newSize;
    }
  });
  return { total, success, failed, totalSaved };
});

function formatSize(bytes: number): string {
  if (bytes === 0) return "0 B";
  const units = ["B", "KB", "MB", "GB", "TB"];
  const k = 1024;
  const i = Math.floor(Math.log(bytes) / Math.log(k));
  const size = (bytes / Math.pow(k, i)).toFixed(i === 0 ? 0 : 1);
  return `${size} ${units[i]}`;
}

function compressionRatio(original: number, newSize: number): string {
  if (original === 0) return "-";
  const ratio = ((1 - newSize / original) * 100).toFixed(0);
  if (Number(ratio) >= 0) return `-${ratio}%`;
  return `+${Math.abs(Number(ratio))}%`;
}

function toggleError(index: number) {
  expandedErrors.value[index] = !expandedErrors.value[index];
}
</script>

<template>
  <div v-if="items.length > 0" class="result-list">
    <!-- 汇总栏 -->
    <div class="result-summary">
      <span>共 {{ stats.total }} 个文件</span>
      <span class="summary-success">✅ {{ stats.success }} 成功</span>
      <span v-if="stats.failed > 0" class="summary-error">❌ {{ stats.failed }} 失败</span>
      <span v-if="stats.totalSaved > 0" class="summary-saved">
        节省 {{ formatSize(stats.totalSaved) }}
      </span>
    </div>

    <!-- 结果列表 -->
    <div v-for="(item, index) in items" :key="index" class="result-item">
      <div class="item-row">
        <el-icon :size="16" :color="item.status === 'success' ? '#67c23a' : '#f56c6c'">
          <CircleCheck v-if="item.status === 'success'" />
          <CircleClose v-else />
        </el-icon>

        <span class="item-name">{{ item.name }}</span>

        <template v-if="item.originalSize !== undefined && item.newSize !== undefined">
          <span class="item-size">
            {{ formatSize(item.originalSize) }} → {{ formatSize(item.newSize) }}
          </span>
          <span
            class="item-ratio"
            :class="item.newSize <= item.originalSize ? 'ratio-down' : 'ratio-up'"
          >
            {{ compressionRatio(item.originalSize, item.newSize) }}
          </span>
        </template>

        <el-icon
          v-if="item.status === 'error' && item.error"
          class="error-toggle"
          :size="16"
          @click="toggleError(index)"
        >
          <InfoFilled />
        </el-icon>
      </div>

      <!-- 错误详情(可展开) -->
      <div v-if="expandedErrors[index] && item.error" class="item-error">
        {{ item.error }}
      </div>
    </div>
  </div>
</template>

<style scoped>
.result-list {
  border: 1px solid var(--el-border-color-light);
  border-radius: 8px;
  overflow: hidden;
}

.result-summary {
  display: flex;
  gap: 16px;
  padding: 10px 16px;
  background-color: var(--el-bg-color);
  border-bottom: 1px solid var(--el-border-color-light);
  font-size: 13px;
  color: var(--el-text-color-regular);
}

.summary-success {
  color: #67c23a;
}

.summary-error {
  color: #f56c6c;
}

.summary-saved {
  color: var(--el-color-primary);
  margin-left: auto;
}

.result-item {
  border-bottom: 1px solid var(--el-border-color-lighter);
}

.result-item:last-child {
  border-bottom: none;
}

.item-row {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 10px 16px;
  font-size: 14px;
}

.item-name {
  font-weight: 500;
}

.item-size {
  margin-left: auto;
  font-size: 12px;
  color: var(--el-text-color-secondary);
  font-variant-numeric: tabular-nums;
}

.item-ratio {
  font-size: 12px;
  font-weight: 600;
  min-width: 40px;
  text-align: right;
  font-variant-numeric: tabular-nums;
}

.ratio-down {
  color: #67c23a;
}

.ratio-up {
  color: #f56c6c;
}

.error-toggle {
  cursor: pointer;
  color: var(--el-text-color-secondary);
  flex-shrink: 0;
}

.error-toggle:hover {
  color: var(--el-color-danger);
}

.item-error {
  padding: 8px 16px 10px 40px;
  font-size: 12px;
  color: #f56c6c;
  background-color: var(--el-color-danger-light-9);
  white-space: pre-wrap;
  word-break: break-all;
}
</style>

<script setup lang="ts">
import { computed } from "vue";
import { CircleCheck, CircleClose, Loading, VideoPause } from "@element-plus/icons-vue";
import type { TaskStatus } from "../types/task";

/**
 * 任务进度条通用组件。
 * - running: 进度条 + 取消按钮
 * - done:    绿色完成态
 * - error:   红色错误态,显示 errorMessage
 * - cancelled: 灰色取消态
 *
 * Props:
 *   progress     — 0-100
 *   status       — TaskStatus
 *   message      — 可选提示文本
 *   errorMessage — 错误详情(当 status === "error" 时展示)
 *
 * Emit:
 *   @cancel — 用户点击取消按钮
 */

const props = withDefaults(
  defineProps<{
    progress: number;
    status: TaskStatus;
    message?: string;
    errorMessage?: string;
  }>(),
  {
    message: "",
    errorMessage: "",
  },
);

const emit = defineEmits<{
  (e: "cancel"): void;
}>();

const statusIcon = computed(() => {
  switch (props.status) {
    case "running":
      return Loading;
    case "done":
      return CircleCheck;
    case "error":
      return CircleClose;
    case "cancelled":
      return VideoPause;
    default:
      return Loading;
  }
});

const statusText = computed(() => {
  switch (props.status) {
    case "running":
      return "处理中...";
    case "done":
      return "完成";
    case "error":
      return "失败";
    case "cancelled":
      return "已取消";
    default:
      return "未知";
  }
});

const progressColor = computed(() => {
  switch (props.status) {
    case "done":
      return "#67c23a";
    case "error":
      return "#f56c6c";
    case "cancelled":
      return "#909399";
    default:
      return undefined; // Element Plus 默认蓝色
  }
});

const progressClass = computed(() => `task-progress-${props.status}`);
</script>

<template>
  <div class="task-progress" :class="progressClass">
    <div class="progress-header">
      <el-icon :size="20" :color="progressColor">
        <component :is="statusIcon" :class="{ 'is-loading': status === 'running' }" />
      </el-icon>
      <span class="status-label">{{ statusText }}</span>
      <span class="status-message">{{ message }}</span>
      <el-button
        v-if="status === 'running'"
        size="small"
        type="warning"
        text
        @click="emit('cancel')"
      >
        取消
      </el-button>
    </div>

    <el-progress
      :percentage="progress"
      :color="progressColor"
      :show-text="status === 'running'"
      :stroke-width="8"
    />

    <el-alert
      v-if="status === 'error' && errorMessage"
      :title="errorMessage"
      type="error"
      :closable="false"
      show-icon
      class="error-alert"
    />
  </div>
</template>

<style scoped>
.task-progress {
  padding: 12px 16px;
  border-radius: 8px;
  background-color: var(--el-bg-color);
  border: 1px solid var(--el-border-color-light);
}

.task-progress-error {
  border-color: var(--el-color-danger-light-5);
  background-color: var(--el-color-danger-light-9);
}

.progress-header {
  display: flex;
  align-items: center;
  gap: 8px;
  margin-bottom: 10px;
}

.status-label {
  font-weight: 600;
  font-size: 14px;
}

.status-message {
  flex: 1;
  font-size: 13px;
  color: var(--el-text-color-secondary);
}

.error-alert {
  margin-top: 10px;
}
</style>

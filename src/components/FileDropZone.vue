<script setup lang="ts">
import { ref, computed } from "vue";
import { UploadFilled } from "@element-plus/icons-vue";

/**
 * 文件拖拽/选择通用组件。
 * 支持拖拽文件到区域 + 点击触发系统文件选择。
 *
 * Props:
 *   accept   — 限制可选文件类型(如 "image/*" / ".pdf")
 *   multiple — 是否允许多选(默认 true)
 *
 * Emit:
 *   @files-selected — 用户选择文件后发出 File[]
 */

const props = withDefaults(
  defineProps<{
    accept?: string;
    multiple?: boolean;
  }>(),
  {
    accept: "*",
    multiple: true,
  },
);

const emit = defineEmits<{
  (e: "files-selected", files: File[]): void;
}>();

const isDragging = ref(false);
const fileInput = ref<HTMLInputElement | null>(null);

const acceptAttr = computed(() => {
  if (!props.accept || props.accept === "*") return undefined;
  return props.accept;
});

let dragCounter = 0;

function onDragEnter(e: DragEvent) {
  e.preventDefault();
  dragCounter++;
  isDragging.value = true;
}

function onDragLeave(e: DragEvent) {
  e.preventDefault();
  dragCounter--;
  if (dragCounter <= 0) {
    dragCounter = 0;
    isDragging.value = false;
  }
}

function onDragOver(e: DragEvent) {
  e.preventDefault();
}

function onDrop(e: DragEvent) {
  e.preventDefault();
  dragCounter = 0;
  isDragging.value = false;

  const files = e.dataTransfer?.files;
  if (!files || files.length === 0) return;

  const filtered = filterFiles(files);
  if (filtered.length > 0) {
    emit("files-selected", filtered);
  }
}

/** 根据 accept 过滤 FileList,返回匹配的文件 */
function filterFiles(fileList: FileList): File[] {
  const result: File[] = [];
  for (let i = 0; i < fileList.length; i++) {
    if (matchAccept(fileList[i])) {
      result.push(fileList[i]);
    }
  }
  return result;
}

/** 检查单个文件是否匹配 accept 规则 */
function matchAccept(file: File): boolean {
  if (!props.accept || props.accept === "*") return true;

  const patterns = props.accept.split(",").map((s) => s.trim());

  return patterns.some((pattern) => {
    if (pattern.startsWith(".")) {
      // 扩展名匹配: ".pdf"
      return file.name.toLowerCase().endsWith(pattern.toLowerCase());
    }
    if (pattern.endsWith("/*")) {
      // MIME 类型组: "image/*"
      return file.type.startsWith(pattern.slice(0, -1));
    }
    // 精确 MIME 匹配
    return file.type === pattern;
  });
}

function onClick() {
  fileInput.value?.click();
}

function onInputChange(e: Event) {
  const input = e.target as HTMLInputElement;
  const files = input.files;
  if (!files || files.length === 0) return;

  const filtered = filterFiles(files);
  if (filtered.length > 0) {
    emit("files-selected", filtered);
  }
  // 重置以允许重复选同一个文件
  input.value = "";
}
</script>

<template>
  <div
    class="file-drop-zone"
    :class="{ dragging: isDragging }"
    @dragenter="onDragEnter"
    @dragleave="onDragLeave"
    @dragover="onDragOver"
    @drop="onDrop"
    @click="onClick"
  >
    <input
      ref="fileInput"
      type="file"
      :accept="acceptAttr"
      :multiple="multiple"
      class="file-input-hidden"
      @change="onInputChange"
    />

    <div class="drop-content">
      <el-icon class="drop-icon" :size="40">
        <UploadFilled />
      </el-icon>
      <p class="drop-text">
        <template v-if="isDragging"> 松开以添加文件 </template>
        <template v-else> 拖拽文件到此处,或点击选择 </template>
      </p>
      <p v-if="accept && accept !== '*'" class="drop-hint">支持格式: {{ accept }}</p>
    </div>
  </div>
</template>

<style scoped>
.file-drop-zone {
  border: 2px dashed var(--el-border-color);
  border-radius: 8px;
  padding: 40px 20px;
  text-align: center;
  cursor: pointer;
  transition:
    border-color 0.3s,
    background-color 0.3s;
  user-select: none;
}

.file-drop-zone:hover,
.file-drop-zone.dragging {
  border-color: var(--el-color-primary);
  background-color: var(--el-color-primary-light-9);
}

.file-input-hidden {
  display: none;
}

.drop-content {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 8px;
}

.drop-icon {
  color: var(--el-text-color-placeholder);
}

.drop-text {
  margin: 0;
  font-size: 14px;
  color: var(--el-text-color-regular);
}

.drop-hint {
  margin: 0;
  font-size: 12px;
  color: var(--el-text-color-secondary);
}
</style>

<template>
  <div style="padding: 20px;">
    <h2>文件批量重命名工具</h2>

    <!-- 文件拖拽区域 -->
    <div
      @drop.prevent="handleDrop"
      @dragover.prevent
      style="border:2px dashed #888; padding:40px; text-align:center; margin:20px 0; border-radius:6px;"
    >
      <p>将需要改名的文件拖拽到此处</p >
      <p v-if="fileList.length > 0">已选择 {{ fileList.length }} 个文件</p >
    </div>

    <!-- 命名规则输入框 -->
    <div style="margin:15px 0;">
      <label>命名规则：</label>
      <input
        v-model="ruleStr"
        placeholder="例如：文档_{index}"
        style="width:320px; padding:8px; margin-left:8px;"
      />
    </div>

    <!-- 预览列表 -->
    <div v-if="previewData.length > 0" style="margin:20px 0;">
      <h4>改名预览</h4>
      <div v-for="(item, idx) in previewData" :key="idx">
        {{ item.oldName }} → {{ item.newName }}
        <span style="color:red" v-if="item.conflict"> 文件名重复</span>
      </div>
    </div>

    <!-- 执行改名按钮 -->
    <button
      @click="startRename"
      :disabled="previewData.some(item => item.conflict) || fileList.length === 0"
      style="padding:8px 16px; cursor:pointer;"
    >
      确认批量改名
    </button>
  </div>
</template>

<script setup>
import { ref, watch, computed } from 'vue'
// Tauri 正确导入路径
import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'

// 绑定变量
const ruleStr = ref('')
const fileList = ref([])
const previewData = ref([])

// 是否存在重名
const hasConflict = computed(() => previewData.value.some(item => item.conflict))

// 拖拽文件接收
const handleDrop = (e) => {
  const paths = Array.from(e.dataTransfer.files).map(file => file.path)
  fileList.value = paths
}

// 规则/文件变化时预览改名结果
watch([ruleStr, fileList], async () => {
  if (!ruleStr.value || fileList.value.length === 0) {
    previewData.value = []
    return
  }
  try {
    previewData.value = await invoke('preview_rename', {
      files: fileList.value,
      rule: ruleStr.value
    })
  } catch (err) {
    console.error('预览失败', err)
    previewData.value = []
  }
})

// 执行改名
const startRename = async () => {
  try {
    await invoke('execute_rename', {
      files: fileList.value,
      rule: ruleStr.value
    })
    alert('批量改名完成')
    fileList.value = []
    previewData.value = []
    ruleStr.value = ''
  } catch (err) {
    alert('改名失败：' + err)
  }
}

// 监听改名进度（可选）
listen('rename_progress', (event) => {
  console.log('改名进度', event.payload)
})
</script>
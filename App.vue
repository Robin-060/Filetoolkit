<template>
  <main class="home">
    <h1>FileToolkit</h1>
    <p class="tagline">一站式本地文件批量处理工具</p >
    <p class="tagline-zh">本地优先 · 隐私安全 · 轻量高效</p >

    <form class="row" @submit.prevent="onGreet">
      <input v-model="name" placeholder="输入名字"/>
      <button type="submit">调用 Rust 后端</button>
    </form>

    <!-- 批量重命名按钮 -->
    <button
      @click="showRenamePanel = !showRenamePanel"
      style="margin:15px 0;padding:8px 16px;cursor:pointer"
    >
      批量重命名
    </button>

    <!-- 点击按钮展开/收起重命名页面 -->
    <div v-if="showRenamePanel">
      <RenamePage />
    </div>

    <p v-if="reply" class="reply">{{ reply }}</p >
  </main>
</template>

<script setup>
import { ref } from 'vue'
// 引入重命名页面
import RenamePage from './pages/RenamePage.vue'

// 原有输入框变量
const name = ref('')
const reply = ref('')

// 控制重命名面板显示隐藏
const showRenamePanel = ref(false)

// 原有调用rust方法
const onGreet = async () => {
  const { invoke } = await import('@tauri-apps/api/core')
  reply.value = await invoke('greet', { name: name.value })
}
</script>

<style scoped>
.home {
  text-align: center;
  padding: 2rem;
}
.row {
  display: flex;
  gap: 10px;
  justify-content: center;
  margin: 20px 0;
}
input {
  padding: 8px 12px;
  font-size: 16px;
}
button {
  padding: 8px 14px;
  cursor: pointer;
}
.tagline {
  font-size: 18px;
  margin: 4px 0;
}
.tagline-zh {
  color: #666;
  margin-bottom: 20px;
}
.reply {
  margin-top: 16px;
  font-size: 17px;
}
</style>
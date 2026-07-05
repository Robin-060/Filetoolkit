<script setup lang="ts">
// M0 调通示例:点击按钮 → 调用 Rust 后端 greet 命令 → 展示返回。
// 后续 M1 将在此基础之上接入各功能页面。
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";

const name = ref("世界");
const reply = ref("");

async function onGreet() {
  reply.value = await invoke<string>("greet", { name: name.value });
}
</script>

<template>
  <main class="home">
    <h1>FileToolkit</h1>
    <p class="tagline">一站式本地文件批量处理工具</p>
    <p class="tagline-zh">本地优先 · 隐私安全 · 轻量高效</p>

    <form class="row" @submit.prevent="onGreet">
      <input v-model="name" placeholder="输入名字" />
      <button type="submit">调用 Rust 后端</button>
    </form>
    <p v-if="reply" class="reply">{{ reply }}</p>
  </main>
</template>

<style scoped>
.home {
  margin: 0;
  padding-top: 12vh;
  display: flex;
  flex-direction: column;
  align-items: center;
  text-align: center;
}

.tagline {
  color: #646cff;
  font-weight: 500;
}
.tagline-zh {
  color: #888;
  margin-top: -0.5em;
}

.row {
  display: flex;
  gap: 8px;
  margin-top: 1.5em;
}

.reply {
  margin-top: 1.5em;
  padding: 0.6em 1.2em;
  border-radius: 8px;
  background-color: #eef7ff;
  color: #0f0f0f;
}
</style>

<style>
:root {
  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  font-size: 16px;
  line-height: 24px;
  font-weight: 400;

  color: #0f0f0f;
  background-color: #f6f6f6;

  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
}

input,
button {
  border-radius: 8px;
  border: 1px solid transparent;
  padding: 0.6em 1.2em;
  font-size: 1em;
  font-weight: 500;
  font-family: inherit;
  color: #0f0f0f;
  background-color: #ffffff;
  transition: border-color 0.25s;
  box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
}

button {
  cursor: pointer;
}

button:hover {
  border-color: #396cd8;
}
button:active {
  border-color: #396cd8;
  background-color: #e8e8e8;
}

input,
button {
  outline: none;
}

@media (prefers-color-scheme: dark) {
  :root {
    color: #f6f6f6;
    background-color: #2f2f2f;
  }

  input,
  button {
    color: #ffffff;
    background-color: #0f0f0f98;
  }
  button:active {
    background-color: #0f0f0f69;
  }
}
</style>

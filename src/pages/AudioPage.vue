<template>
  <div class="audio-box">
    <h2>音频处理工具</h2>
    <button @click="audioConvert">格式转换</button>
    <button @click="audioCut">音频剪切</button>
    <button @click="audioMerge">合并音频</button>
    <button @click="audioNormalize">音量标准化</button>
  </div>
</template>

<script setup>
import { invoke } from '@tauri-apps/api/invoke'

// 音频格式转换
async function audioConvert() {
  await invoke('convert_audio', {
    input: '/文件完整路径/input.wav',
    format: 'mp3',
    bitrate: '320k',
    output: '/输出路径/out.mp3'
  })
  alert("转换完成")
}

// 音频剪切
async function audioCut() {
  await invoke('cut_audio', {
    input: '/test.mp3',
    start: '00:00:05',
    end: '00:00:20',
    output: '/cut.mp3'
  })
  alert("剪切完成")
}

// 音频合并
async function audioMerge() {
  await invoke('merge_audio', {
    files: ['/a1.mp3', '/a2.mp3'],
    output: '/all_audio.mp3'
  })
  alert("合并完成")
}

// 音量标准化
async function audioNormalize() {
  await invoke('normalize_audio', {
    input: '/loud.wav',
    targetLufs: -16,
    output: '/standard.wav'
  })
  alert("音量标准化完成")
}
</script>
<template>
  <n-modal v-model:show="visible" preset="card" title="配置预览" style="width: 600px">
    <ConfigPreview :config="config" />
  </n-modal>
</template>

<script lang="ts" setup>
import { ref } from 'vue'
import { NModal } from 'naive-ui'
import ConfigPreview from './ConfigPreview.vue'

const visible = ref(false)
const config = ref()

function showPreview(currentConfig: Config) { // 使用具体的Config类型
  config.value = JSON.parse(JSON.stringify(currentConfig)) // 解除Proxy响应式
  visible.value = true
}

defineExpose({
  showPreview
})
</script>

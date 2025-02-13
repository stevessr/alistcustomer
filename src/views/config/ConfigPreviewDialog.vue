<template>
  <n-modal v-model:show="visible" preset="card" title="配置预览" style="width: 600px">
    <ConfigPreview :config="currentConfig" />
  </n-modal>
</template>

<script lang="ts" setup>
import { ref } from 'vue'
import { NModal } from 'naive-ui'
import ConfigPreview from './ConfigPreview.vue'
import { Config } from './father.vue'

const visible = ref(false)

const currentConfig = ref()

function showPreview(config: Config) { // 使用具体的Config类型
  currentConfig.value = JSON.parse(JSON.stringify(config)) // 解除Proxy响应式
  visible.value = true
}

defineExpose({
  showPreview
})
</script>

<template>
  <BaseLayout>
    <template #header>
      <h1>下载 AList</h1>
    </template>

    <div class="download-container">
      <el-form :model="form" label-width="120px">
        <el-form-item label="代理地址">
          <el-input v-model="form.proxy" placeholder="可选，如 http://127.0.0.1:8080" />
        </el-form-item>

        <el-form-item>
          <el-button 
            type="primary"
            :loading="loading"
            @click="handleDownload"
          >
            下载并解压
          </el-button>
        </el-form-item>
      </el-form>
    </div>
  </BaseLayout>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/tauri'

const form = ref({
  proxy: ''
})

const loading = ref(false)

async function handleDownload() {
  try {
    loading.value = true
    await invoke('download_and_extract_alist', { 
      proxyUrl: form.value.proxy || null 
    })
  } catch (error) {
    console.error(error)
  } finally {
    loading.value = false
  }
}
</script>

<style scoped>
.download-container {
  max-width: 800px;
  margin: 0 auto;
  padding: 20px;
}
</style>

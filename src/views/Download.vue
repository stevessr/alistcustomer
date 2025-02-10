<template>
  <BaseLayout>
    <div class="download-container">
      <n-button @click="showOptions = true" secondary style="margin-bottom: 16px">
        可选参数
      </n-button>
      <n-form :model="form" label-placement="left" label-width="120px">
        <n-form-item label="版本">
          <n-select
            v-model:value="form.version"
            placeholder="请选择版本"
            :options="[
              { label: '最新版', value: 'latest' },
              { label: 'v3.28.0', value: 'v3.28.0' },
              { label: 'v3.27.0', value: 'v3.27.0' }
            ]"
          />
        </n-form-item>

        <n-form-item label="代理地址">
          <n-input
            v-model:value="form.proxy"
            placeholder="可选，如 http://127.0.0.1:8080"
          />
        </n-form-item>

        <n-form-item>
          <n-progress
            v-if="progress > 0"
            type="line"
            :percentage="progress"
            :status="progress === 100 ? 'success' : 'default'"
            class="download-progress"
          />
          <n-button
            type="primary"
            :loading="loading"
            @click="handleDownload"
          >
            下载并解压
          </n-button>
        </n-form-item>
  </n-form>

      <n-modal :show="showOptions" @update:show="(val: boolean) => showOptions = val" title="可选参数">
        <n-card style="width: 400px">
          <n-space vertical>
            <n-checkbox v-model:checked="useProxy">使用代理</n-checkbox>
            <n-input
              v-model="proxyUrl"
              placeholder="请输入代理URL"
              :disabled="!useProxy"
            />
            <n-input
              v-model="proxyUsername"
              placeholder="请输入代理用户名"
              :disabled="!useProxy"
            />
            <n-input
              v-model="proxyPassword"
              placeholder="请输入代理密码"
              :disabled="!useProxy"
              type="password"
            />
            <div style="text-align: center">
              <n-button @click="showOptions = false">关闭</n-button>
              <n-button @click="deleteDataFolder">删除数据文件夹</n-button>
            </div>
          </n-space>
        </n-card>
      </n-modal>
    </div>
  </BaseLayout>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { useMessage } from 'naive-ui'

const message = useMessage()
const form = ref({
  proxy: '',
  version: 'latest'
})

const showOptions = ref(false)
const useProxy = ref(false)
const proxyUrl = ref('')
const proxyUsername = ref('')
const proxyPassword = ref('')

const deleteDataFolder = async () => {
  try {
    await invoke('delete_data_folder')
    message.success('数据文件夹删除成功')
  } catch (error) {
    message.error('删除数据文件夹失败')
    console.error(error)
  }
}

const loading = ref(false)
const progress = ref(0)

async function handleDownload() {
  try {
    loading.value = true
    progress.value = 0
    
    await invoke('download_and_extract_alist', { 
      proxyUrl: form.value.proxy || null,
      version: form.value.version
    })
    
    progress.value = 100
    message.success('下载并解压成功！')
  } catch (error: unknown) {
    console.error(error)
    if (error instanceof Error) {
      message.error(`下载失败: ${error.message}`)
    } else {
      message.error('下载失败: 未知错误')
    }
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

.download-progress {
  margin-bottom: 20px;
}
</style>

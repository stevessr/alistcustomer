<template>
  <div class="config-preview">
    <div class="preview-header">
      <h3 class="preview-title">配置预览</h3>
      <button 
        class="preview-mode-btn"
        @click="togglePreviewMode"
      >
        {{ isRawMode ? '表格视图' : '原始数据' }}
      </button>
    </div>
    <div v-if="!isRawMode" class="preview-content">
      <div class="preview-item" 
           v-for="(item, index) in previewData" 
           :key="index"
           :style="{ backgroundColor: item.color }">
        <span class="item-label">{{ item.label }}:</span>
        <span class="item-value">{{ item.value }}</span>
      </div>
      <button class="random-color-btn" @click="randomizeColors(previewData)">
        随机颜色
      </button>
    </div>
    <pre v-else class="raw-preview">{{ rawConfig }}</pre>
  </div>
</template>

<script lang="ts" setup>
import { computed, ref } from 'vue'

// Define random color generator function
const randomizeColors = (data: PreviewItem[]) => {
  const letters = '0123456789ABCDEF'
  data.forEach(item => {
    let color = '#'
    for (let i = 0; i < 6; i++) {
      color += letters[Math.floor(Math.random() * 16)]
    }
    item.color = color
  })
}

// Update PreviewItem interface to include color
interface PreviewItem {
  label: string
  value: string | number | boolean
  color?: string
}

const props = defineProps<{
  config: any
}>()

const isRawMode = ref(false)

const rawConfig = computed(() => {
  return JSON.stringify(props.config, null, 2)
})

function togglePreviewMode() {
  isRawMode.value = !isRawMode.value
}

const previewData = computed(() => {
  if (!props.config) return []
  
  const data: PreviewItem[] = []
  randomizeColors(data)
  
  // General Config
  if (props.config.general) {
    data.push(
      { label: '站点URL', value: props.config.general.site_url || '未设置' },
      { label: 'CDN地址', value: props.config.general.cdn || '未设置' },
      { label: '强制HTTPS', value: props.config.general.force ? '是' : '否' }
    )
  }
  
  // Database Config
  if (props.config.database) {
    data.push(
      { label: '数据库类型', value: props.config.database.type || '未设置' },
      { label: '数据库地址', value: props.config.database.host || '未设置' },
      { label: '数据库端口', value: props.config.database.port || '未设置' }
    )
  }

  // Meilisearch Config
  if (props.config.meilisearch) {
    data.push(
      { label: 'Meilisearch服务', value: props.config.meilisearch.enable ? '启用' : '禁用' },
      { label: 'API地址', value: props.config.meilisearch.api_url || '未设置' },
      { label: 'API密钥', value: props.config.meilisearch.api_key ? '已设置' : '未设置' }
    )
  }

  // Scheme Config
  if (props.config.scheme) {
    data.push(
      { label: '默认方案', value: props.config.scheme.default || '未设置' },
      { label: '允许的方案', value: props.config.scheme.allowed?.join(', ') || '未设置' }
    )
  }

  // Log Config
  if (props.config.log) {
    data.push(
      { label: '日志级别', value: props.config.log.level || '未设置' },
      { label: '日志路径', value: props.config.log.path || '未设置' }
    )
  }

  // Task Config
  if (props.config.task) {
    data.push(
      { label: '任务队列', value: props.config.task.queue || '未设置' },
      { label: '最大并发数', value: props.config.task.max_concurrent || '未设置' }
    )
  }

  // Cors Config
  if (props.config.cors) {
    data.push(
      { label: '允许的来源', value: props.config.cors.allow_origins?.join(', ') || '未设置' },
      { label: '允许的方法', value: props.config.cors.allow_methods?.join(', ') || '未设置' }
    )
  }

  // S3 Config
  if (props.config.s3) {
    data.push(
      { label: 'S3服务', value: props.config.s3.enable ? '启用' : '禁用' },
      { label: 'S3端口', value: props.config.s3.port },
      { label: 'SSL加密', value: props.config.s3.ssl ? '启用' : '禁用' }
    )
  }

  // FTP Config
  if (props.config.ftp) {
    data.push(
      { label: 'FTP服务', value: props.config.ftp.enable ? '启用' : '禁用' },
      { label: '监听地址', value: props.config.ftp.listen },
      { label: '空闲超时', value: `${props.config.ftp.idle_timeout}秒` }
    )
  }

  // SFTP Config
  if (props.config.sftp) {
    data.push(
      { label: 'SFTP服务', value: props.config.sftp.enable ? '启用' : '禁用' },
      { label: '监听地址', value: props.config.sftp.listen }
    )
  }

  return data
})

// Expose randomizeColors to template
defineExpose({
  randomizeColors
})
</script>

<style scoped>
.preview-title {
  font-size: 16px;
  font-weight: 500;
  margin-bottom: 16px;
  color: var(--n-text-color);
}

.preview-content {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.preview-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 16px;
}

.preview-mode-btn {
  padding: 4px 12px;
  border-radius: 4px;
  border: 1px solid var(--n-border-color);
  background: var(--n-color-embedded);
  cursor: pointer;
  font-size: 14px;
}

.preview-mode-btn:hover {
  background: var(--n-color-embedded-hover);
}

.preview-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 8px 12px;
  background: var(--n-color-embedded);
  border-radius: 6px;
}

.raw-preview {
  padding: 12px;
  background: var(--n-color-embedded);
  border-radius: 6px;
  max-height: 400px;
  overflow: auto;
  white-space: pre-wrap;
  word-wrap: break-word;
  font-family: monospace;
  font-size: 14px;
  line-height: 1.5;
}

.item-label {
  font-size: 14px;
  color: var(--n-text-color);
}

.item-value {
  font-size: 14px;
  font-weight: 500;
  color: var(--n-text-color-secondary);
}
</style>

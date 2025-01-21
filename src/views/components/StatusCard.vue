<script setup lang="ts">
import { ref } from "vue";
import Change_password from "./change_password.vue";

defineProps<{
  status: {
    running: boolean;
    pid?: number;
  };
  message: string;
  versionInfo?: {
    version: string;
    web_version: string;
    build_date?: string;
    commit_hash?: string;
    commit_id?: string;
    platform?: string;
  } | null;
  loading: boolean;
  showVersionDialog?: boolean;
}>();

const emit = defineEmits<{
  (e: 'refresh'): void;
  (e: 'start'): void;
  (e: 'stop'): void;
  (e: 'download'): void;
  (e: 'getVersion'): void;
}>();
</script>

<template>
  <n-card class="status-card">
    <n-space vertical>
      <n-alert :type="status.running ? 'success' : 'error'">
        当前 alist 状态：{{ status?.running ? "运行中" : "已停止" }}
        <template #icon>
          <n-icon :name="status?.running ? 'checkmark-circle' : 'close-circle'" />
        </template>
      </n-alert>

      <n-descriptions label-placement="left" bordered>
        <n-descriptions-item label="进程 ID">
          {{ status?.pid || "无" }}
        </n-descriptions-item>
        <n-descriptions-item label="状态信息">
          {{ message }}
        </n-descriptions-item>
        <n-descriptions-item label="版本信息" v-if="versionInfo">
          <n-space vertical>
            <n-tag type="info" size="small">核心版本: {{ versionInfo?.version }}</n-tag>
            <n-tag type="info" size="small">Web版本: {{ versionInfo?.web_version }}</n-tag>
            <n-tag v-if="versionInfo?.build_date" type="info" size="small">
              构建日期: {{ versionInfo?.build_date }}
            </n-tag>
            <n-tag v-if="versionInfo?.commit_hash" type="info" size="small">
              Git提交: {{ versionInfo?.commit_hash?.slice(0, 7) }}
            </n-tag>
            <n-tag v-if="versionInfo?.platform" type="info" size="small">
              平台: {{ versionInfo?.platform }}
            </n-tag>
          </n-space>
        </n-descriptions-item>
      </n-descriptions>

      <n-space justify="center">
        <n-button-group>
          <n-button 
            @click="emit('refresh')" 
            type="primary" 
            :loading="loading"
          >
            刷新状态
          </n-button>
          <n-button 
            @click="emit('start')"
            :disabled="status?.running || loading"
            :loading="loading"
            v-if="!status?.running"
            type="success"
          >
            启动 alist
          </n-button>
          <n-button 
            @click="emit('stop')"
            :disabled="!status?.running || loading"
            :loading="loading"
            v-if="status?.running"
            type="error"
          >
            停止 alist
          </n-button>
        </n-button-group>
      </n-space>

      <n-space justify="center" class="additional-actions">
        <n-button @click="emit('getVersion')" secondary :loading="loading">
          获取版本信息
        </n-button>
        <n-button 
          @click="emit('download')"
          :disabled="status.running || loading"
          :loading="loading"
          v-if="!status.running"
          secondary
        >
          下载 alist
        </n-button>
      </n-space>

      <Change_password class="change-password" />
    </n-space>
  </n-card>
</template>

<style scoped>
.status-card {
  margin: 20px;
}

.additional-actions {
  margin-top: 20px;
}

.change-password {
  margin-top: 20px;
}
</style>

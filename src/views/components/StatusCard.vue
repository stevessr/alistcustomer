<script setup lang="ts">
import { ref, toRef } from "vue";
import { createGetAlistVersion } from "../status/api.ts";
import type { AlistVersionInfo } from "../../types/alist";

const props = defineProps<{
  status: {
    running: boolean;
    pid?: number;
  };
  message: string;
  versionInfo?: AlistVersionInfo | null;
  loading: boolean;
  showVersionDialog?: boolean;
}>();

const status = ref(props.status);

const emit = defineEmits<{
  (e: 'refresh'): void;
  (e: 'start'): void;
  (e: 'stop'): void;
  (e: 'getVersion'): void;
}>();

const versionInfoRef = ref<AlistVersionInfo | null>(props.versionInfo ?? null);
const messageRef = ref<string>(props.message || '');

const getAlistVersion = createGetAlistVersion(versionInfoRef, messageRef);
const isGettingVersion = ref(false);

async function updateVersionInfo() {
  isGettingVersion.value = true;
  try {
    await getAlistVersion();
    // 同步父组件状态
    emit('getVersion');
  } catch (error) {
    messageRef.value = error instanceof Error ? error.message : '获取版本信息失败';
  } finally {
    isGettingVersion.value = false;
  }
}
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
            :disabled="status.running || loading"
            :loading="loading"
            v-if="!status.running"
            type="success"
          >
            启动 alist
          </n-button>
          <n-button 
            @click="emit('stop')"
            :disabled="!status.running || loading"
            :loading="loading"
            v-if="status.running"
            type="error"
          >
            停止 alist
          </n-button>
        </n-button-group>
      </n-space>
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

</style>

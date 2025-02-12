<script setup lang="ts">
import { computed , onMounted  } from "vue";
import type { AlistVersionInfo } from "../../types/alist";

interface ProcessStatus {
  running: boolean;
  pid?: number;
}

interface Props {
  status: ProcessStatus;
  message: string;
  versionInfo?: AlistVersionInfo | null;
  loading: boolean;
  showVersionDialog?: boolean;
}

const props = withDefaults(defineProps<Props>(), {
  showVersionDialog: false,
  versionInfo: undefined, // 明确声明当不传时的默认值
});

const status = computed(() => props.status);

const emit = defineEmits<{
  (e: "refresh"): void;
  (e: "start"): void;
  (e: "stop"): void;
  (e: "getVersion"): void;
}>();

onMounted(() => {
  emit('refresh')
})
</script>

<template>
  <n-card class="status-card">
    <n-space vertical>
      <n-alert :type="status.running ? 'success' : 'error'">
        当前 alist 状态：{{ status?.running ? "运行中" : "已停止" }}
        <template #icon>
          <n-icon
            :name="status?.running ? 'checkmark-circle' : 'close-circle'"
          />
        </template>
      </n-alert>

      <n-descriptions label-placement="left" bordered>
        <n-descriptions-item label="进程 ID">
          {{ status?.pid || "无" }}
        </n-descriptions-item>
        <n-descriptions-item label="状态信息" v-if="message">
          {{ message }}
        </n-descriptions-item>
      </n-descriptions>

      <n-space justify="center">
        <n-button-group>
          <n-button @click="emit('refresh')" type="primary" :loading="loading">
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

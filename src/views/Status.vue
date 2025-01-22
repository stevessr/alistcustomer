<script setup lang="ts">
import { computed, onMounted, onUnmounted, ref, toRefs } from "vue";
import BaseLayout from "./components/BaseLayout.vue";
import StatusCard from "./components/StatusCard.vue";
import { useStatus } from "./status/status";
import { useAlistApi } from "../composables/useAlistApi";
import type { AlistStatus, AlistVersionInfo, StatusHistoryRecord } from "../types/alist";

const statusStore = useStatus();
const {
  status,
  message,
  loading,
  showOptions,
  showVersionDialog,
  useProxy,
  proxyUrl,
  proxyUsername,
  proxyPassword
} = toRefs(statusStore);
const versionInfo = statusStore.versionInfo;

const api = ref(useAlistApi());
const uptime = ref(0);
const formattedUptime = computed(() => {
  const seconds = uptime.value;
  const days = Math.floor(seconds / (3600 * 24));
  const hours = Math.floor((seconds % (3600 * 24)) / 3600);
  const minutes = Math.floor((seconds % 3600) / 60);
  const secs = Math.floor(seconds % 60);
  
  return [
    days > 0 ? `${days}天` : '',
    hours > 0 ? `${hours}小时` : '',
    minutes > 0 ? `${minutes}分钟` : '',
    `${secs}秒`
  ].filter(Boolean).join(' ') || '0秒';
});

const cpuUsage = ref(0);
const memoryUsage = ref(0);
const statusHistory = ref<StatusHistoryRecord[]>([]);
const errorLogs = ref<string[]>([]);
const startupLogs = ref<string[]>([]);

// Listen for alist logs
onMounted(() => {
  window.addEventListener('alist-log', (event: any) => {
    const log = event.detail;
    startupLogs.value.push(`[${new Date().toLocaleString()}] ${log}`);
  });
});

const handleGetVersion = async () => {
  await api.value.getAlistVersion();
  showVersionDialog.value = true;
}

const updateStatusValues = (metrics: any) => {
  uptime.value = metrics.uptime;
  cpuUsage.value = metrics.cpuUsage;
  memoryUsage.value = metrics.memoryUsage;
  
  if (status.value.running !== statusHistory.value[0]?.status) {
    statusHistory.value.unshift({
      type: status.value.running ? 'success' : 'error',
      title: status.value.running ? '服务启动' : '服务停止', 
      content: message.value,
      time: new Date().toLocaleString(),
      status: status.value.running
    });
    
    if (statusHistory.value.length > 50) {
      statusHistory.value.pop();
    }
  }
}

const updateMetrics = async () => {
  try {
    const metrics = await api.value.getMetrics();
    updateStatusValues(metrics);
  } catch (error: unknown) {
    const errorMessage = error instanceof Error ? error.message : 'Unknown error';
    errorLogs.value.push(`[${new Date().toLocaleString()}] ${errorMessage}`);
    console.error('Failed to update metrics:', error);
  }
};

onMounted(() => {
  api.value.startPolling();
  const metricsInterval = setInterval(updateMetrics, 5000);
  
  onUnmounted(() => {
    clearInterval(metricsInterval);
  });
});

onUnmounted(() => {
  api.value.stopPolling();
});
</script>

<template>
  <BaseLayout>
    <template #header>
      <h1>Status Page</h1>
      <p>这是状态页面</p>
      <n-space vertical>
        <n-statistic label="运行时间" :value="formattedUptime" />
        <n-progress
          type="line"
          :percentage="cpuUsage"
          status="success"
          :indicator-placement="'inside'"
        >
          CPU 使用率: {{ cpuUsage }}%
        </n-progress>
        <n-progress
          type="line"
          :percentage="memoryUsage"
          status="warning"
          :indicator-placement="'inside'"
        >
          内存使用率: {{ memoryUsage }}%
        </n-progress>
      </n-space>
    </template>

    <n-tabs type="line" animated>
      <n-tab-pane name="status" tab="当前状态">
        <StatusCard
          :class="{ 'status-card': true, 'loading': loading }"
          :status="status"
          :message="message"
          :version-info="versionInfo"
          :loading="loading"
          @refresh="async () => await api?.getAlistStatus()"
          @start="async () => await api?.startAlist()"
          @stop="async () => await api?.stopAlist()"
          @download="async () => await api?.downloadAlist()"
          @getVersion="handleGetVersion"
          :show-version-dialog="showVersionDialog"
          aria-live="polite"
          aria-busy="loading"
        />

        <n-button @click="showOptions = true" secondary>
          可选参数
        </n-button>

        <n-modal v-model:show="showOptions" title="可选参数">
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
                <n-button @click="async () => await api?.deleteDataFolder()">删除数据文件夹</n-button>
              </div>
            </n-space>
          </n-card>
        </n-modal>

        <n-modal v-model:show="showVersionDialog" title="版本信息">
          <n-card style="width: 600px">
            <n-space vertical>
              <n-descriptions label-placement="left" bordered>
                <n-descriptions-item label="核心版本">
                  {{ versionInfo?.version || '未知' }}
                </n-descriptions-item>
                <n-descriptions-item label="Web版本">
                  {{ versionInfo?.web_version || '未知' }}
                </n-descriptions-item>
                <n-descriptions-item label="构建日期">
                  {{ versionInfo?.built_at || '未知' }}
                </n-descriptions-item>
                <n-descriptions-item label="Go版本">
                  {{ versionInfo?.go_version || '未知' }}
                </n-descriptions-item>
                <n-descriptions-item label="作者">
                  {{ versionInfo?.author || '未知' }}
                </n-descriptions-item>
                <n-descriptions-item label="Commit ID">
                  {{ versionInfo?.commit_id || '未知' }}
                </n-descriptions-item>
              </n-descriptions>
              <div style="text-align: center">
                <n-button @click="showVersionDialog = false">关闭</n-button>
              </div>
            </n-space>
          </n-card>
        </n-modal>
      </n-tab-pane>

      <n-tab-pane name="history" tab="状态历史">
        <n-timeline>
          <n-timeline-item
            v-for="(record, index) in statusHistory"
            :key="index"
            :type="record.type"
            :title="record.title"
            :content="record.content"
            :time="record.time"
          />
        </n-timeline>
      </n-tab-pane>

      <n-tab-pane name="error-logs" tab="错误日志">
        <n-log :log="errorLogs.join('\n')" />
      </n-tab-pane>

      <n-tab-pane name="startup-logs" tab="启动日志">
        <n-log :log="startupLogs.join('\n')" />
      </n-tab-pane>
    </n-tabs>
  </BaseLayout>
</template>

<style scoped>
@import "./status/status.css";
</style>

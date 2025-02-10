<template>
  <BaseLayout>
    <n-tabs type="line" animated>
      <n-tab-pane name="status" tab="当前状态">
        <StatusCard
          :class="{ 'status-card': true, 'loading': loading }"
          :status="status"
          :message="message"
          :version-info="versionInfo"
          :loading="loading"
          @refresh="async () => {
            loading.value = true;
            try {
              const result = await api.getAlistStatus();
              if (result && typeof result === 'object') {
                status.running = result.running || false;
                status.pid = result.pid;
              }
            } catch (error) {
              message = '刷新状态失败，请检查控制台';
              status.running = false;
              status.pid = undefined;
            } finally {
              loading = false;
            }
          }"
          @start="async () => await api.startAlist()"
          @stop="async () => await api.stopAlist()"
          @download="async () => await api.downloadAlist()"
          @getVersion="handleGetVersion"
          :show-version-dialog="showVersionDialog"
          aria-live="polite"
          aria-busy="loading"
        />

        <n-button @click="handleCheckStatus" type="primary">
          检查系统状态
        </n-button>

      <n-tab-pane name="version" tab="版本信息">
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
              <n-button @click="handleGetVersion" type="primary">获取版本信息</n-button>
            </div>
          </n-space>
        </n-card>
      </n-tab-pane>
      </n-tab-pane>
    </n-tabs>
  </BaseLayout>
</template>

<script setup lang="ts">
import { onMounted, onUnmounted, ref } from "vue";
import BaseLayout from "./components/BaseLayout.vue";
import StatusCard from "./components/StatusCard.vue";
import { useStatus } from "./status/status";
import { useAlistApi } from "../composables/useAlistApi";

const statusStore = useStatus();
const { message, loading, showVersionDialog } = statusStore;
const status = ref<{ running: boolean; pid: number | undefined }>({ running: false, pid: undefined });
const versionInfo = ref(statusStore.versionInfo);
const api = ref(useAlistApi());

onMounted(() => {
  api.value.startPolling();
});

onUnmounted(() => {
  api.value.stopPolling();
});

const handleGetVersion = async () => {
  try {
            loading.value = true;
    message.value = '获取版本信息中...';
    const versionData = await api.value.getAlistVersion();
    versionInfo.value = versionData;
    showVersionDialog.value = true;
  } catch (error) {
    console.error('获取版本信息失败:', error);
    message.value = '获取版本信息失败，请检查控制台';
    showVersionDialog.value = false;
    versionInfo.value = null;
  } finally {
    loading.value = false;
  }
}

const handleCheckStatus = async () => {
  try {
    loading.value = true;
    const systemStatus = await statusStore.checkSystemStatus();
    status.value.running = systemStatus.running;
    status.value.pid = systemStatus.pid ?? undefined;
    message.value = systemStatus.running ? '系统运行正常' : '系统出现问题';
  } catch (error) {
    console.error('Failed to check system status:', error);
    message.value = '检查系统状态失败';
  } finally {
    loading.value = false;
  }
}
</script>

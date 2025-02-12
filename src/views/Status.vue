<template>
  <BaseLayout>
    <n-tabs type="line" animated>
      <n-tab-pane name="status" tab="当前状态">
        <StatusCard
          :class="{ 'status-card': true, loading: loading }"
          :status="status"
          :message="message"
          :version-info="versionInfo"
          :loading="loading"
          @refresh="handleRefresh"
          @start="handleStart"
          @stop="handleStop"
          @getVersion="handleGetVersion"
          :show-version-dialog="showVersionDialog"
          aria-live="polite"
          aria-busy="loading"
        />

        <n-button @click="handleCheckStatus" type="primary">
          检查系统状态
        </n-button>
      </n-tab-pane>

      <n-tab-pane name="version" tab="版本信息">
        <n-card style="width: 600px">
          <n-space vertical>
            <n-descriptions label-placement="left" bordered>
              <n-descriptions-item label="核心版本">
                {{ versionInfo?.version || "未知" }}
              </n-descriptions-item>
              <n-descriptions-item label="Web版本">
                {{ versionInfo?.web_version || "未知" }}
              </n-descriptions-item>
              <n-descriptions-item label="构建日期">
                {{ versionInfo?.built_at || "未知" }}
              </n-descriptions-item>
              <n-descriptions-item label="Go版本">
                {{ versionInfo?.go_version || "未知" }}
              </n-descriptions-item>
              <n-descriptions-item label="作者">
                {{ versionInfo?.author || "未知" }}
              </n-descriptions-item>
              <n-descriptions-item label="Commit ID">
                {{ versionInfo?.commit_id || "未知" }}
              </n-descriptions-item>
            </n-descriptions>
            <div style="text-align: center">
              <n-button @click="handleGetVersion" type="primary"
                >获取版本信息</n-button
              >
            </div>
          </n-space>
        </n-card>
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
import type { AlistStatus } from "../types/alist";

const statusStore = useStatus();
const { showVersionDialog } = statusStore;
const message = ref(statusStore.message);
const loading = ref(false);
const defaultStatus: AlistStatus = { running: false, pid: undefined };
const status = ref<AlistStatus>(defaultStatus); // Ensure this is correctly typed
const versionInfo = ref(statusStore.versionInfo);
const api = useAlistApi();

const errorHandler = (error: unknown, defaultMessage: string) => {
  console.error(error);
  message.value = defaultMessage;
  return false;
};

const withLoading = async (fn: () => Promise<void>) => {
  loading.value = true;
  try {
    await fn();
  } finally {
    loading.value = false;
  }
};

const handleRefresh = async () => {
  await withLoading(async () => {
    try {
      const result = await api.getAlistStatus();
      // Ensure result is valid before accessing properties
      status.value = {
        running: result?.running ?? false,
        pid: result?.pid,
      };
    } catch (error) {
      errorHandler(error, "刷新状态失败，请检查控制台");
      status.value = { running: false, pid: undefined };
    }
  });
};

const handleStart = async () => {
  await withLoading(async () => {
    try {
      await api.startAlist();
      await handleRefresh(); // Ensure this is called after starting Alist
    } catch (error) {
      errorHandler(error, "启动失败，请检查控制台");
    }
  });
};

const handleStop = async () => {
  await withLoading(async () => {
    try {
      await api.stopAlist();
      await handleRefresh();
    } catch (error) {
      errorHandler(error, "停止失败，请检查控制台");
    }
  });
};

const handleGetVersion = async () => {
  await withLoading(async () => {
    try {
      versionInfo.value = await api.getAlistVersion();
      showVersionDialog.value = true;
    } catch (error) {
      errorHandler(error, "获取版本信息失败，请检查控制台");
      showVersionDialog.value = false;
      versionInfo.value = null;
    }
  });
};

const handleCheckStatus = async () => {
  await withLoading(async () => {
    try {
      const systemStatus = await statusStore.checkSystemStatus();
      // Ensure systemStatus is valid before accessing properties
      status.value =
        systemStatus || ({ running: false, pid: undefined } as AlistStatus);
      message.value = systemStatus?.running ? "系统运行正常" : "系统出现问题";
    } catch (error) {
      errorHandler(error, "检查系统状态失败");
    }
  });
};

onMounted(() => {
  api.startPolling();
  handleGetVersion();
});
onUnmounted(() => api.stopPolling());
</script>

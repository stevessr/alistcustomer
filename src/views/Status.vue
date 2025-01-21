<script setup lang="ts">
import { onMounted, onUnmounted, ref, toRefs } from "vue";
import { invoke } from "@tauri-apps/api/core";
import BaseLayout from "./components/BaseLayout.vue";
import Change_password from "./components/change_password.vue";
import StatusCard from "./components/StatusCard.vue";
import { useStatus } from "./status/status";
import { useAlistApi } from "../composables/useAlistApi";
import type { AlistStatus, AlistVersionInfo } from "../types/alist";

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

onMounted(() => {
  api.value.startPolling();
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
    </template>

      <StatusCard
      :status="status"
      :message="message"
      :version-info="versionInfo"
      :loading="loading"
      @refresh="async () => await api?.getAlistStatus()"
      @start="async () => await api?.startAlist()"
      @stop="async () => await api?.stopAlist()"
      @download="async () => await api?.downloadAlist()"
      @getVersion="async () => {
        await api.getAlistVersion();
        showVersionDialog = true;
      }"
      :show-version-dialog="showVersionDialog"
    />

    <n-button @click="showOptions = true" secondary>
      可选参数
    </n-button>

    <!-- 可选参数菜单 -->
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

    <!-- 版本信息对话框 -->
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

  </BaseLayout>
</template>

<style scoped>
@import "./status/status.css";
</style>

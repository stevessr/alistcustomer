<script setup lang="ts">
import { onMounted, onUnmounted, ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { Store } from "@tauri-apps/plugin-store";
import BaseLayout from "./components/BaseLayout.vue";
import Change_password from "./components/change_password.vue";
import { useStatus } from "./status/status";
import type { AlistStatus, AlistVersionInfo } from "@/types/alist";

const { 
  status, 
  message, 
  loading,
  showOptions, 
  showVersionDialog, 
  versionInfo,
  useProxy,
  proxyUrl,
  proxyUsername,
  proxyPassword
} = useStatus();

interface AlistApi {
  startPolling: () => void;
  stopPolling: () => void;
  getAlistStatus: () => Promise<void>;
  startAlist: () => Promise<void>;
  stopAlist: () => Promise<void>;
  downloadAlist: () => Promise<void>;
  getAlistVersion: () => Promise<void>;
  deleteDataFolder: () => Promise<void>;
}

const api = ref<Awaited<ReturnType<typeof useAlistApi>>>();

async function useAlistApi(): Promise<{
  startPolling: () => void;
  stopPolling: () => void;
  getAlistStatus: () => Promise<void>;
  startAlist: () => Promise<void>;
  stopAlist: () => Promise<void>;
  downloadAlist: () => Promise<void>;
  getAlistVersion: () => Promise<void>;
  deleteDataFolder: () => Promise<void>;
}> {
  const alistStatusStore = await Store.load("alist-status.store");
  const {
    status,
    message,
    proxyUrl,
    proxyUsername,
    proxyPassword,
    useProxy,
    versionInfo
  } = useStatus();

  let pollTimer: number | null = null;

  function startPolling() {
    stopPolling();
    pollTimer = window.setInterval(() => {
      getAlistStatus();
    }, 2000);
  }

  function stopPolling() {
    if (pollTimer !== null) {
      window.clearInterval(pollTimer);
      pollTimer = null;
    }
  }

  async function getAlistStatus() {
    try {
      loading.status.value = true;
      await invoke("manage_alist_state");
      
      const statusResult = await invoke<AlistStatus>("get_alist_status");
      if (!statusResult) {
        throw new Error("Failed to get alist status");
      }
      await alistStatusStore.set("status", statusResult);
      const storedStatus = await alistStatusStore.get<AlistStatus>("status");
      if (storedStatus) {
        status.value = {
          running: storedStatus.running,
          pid: storedStatus.pid
        };
      }
      message.value = "状态获取成功！";
    } catch (error) {
      message.value = `获取状态失败：${error}`;
    } finally {
      loading.status.value = false;
    }
  }

  async function startAlist() {
    try {
      loading.start.value = true;
      status.value = await invoke("start_alist");
      message.value = "alist 启动成功！";
    } catch (error) {
      message.value = `启动失败：${error}`;
    } finally {
      loading.start.value = false;
    }
  }

  async function stopAlist() {
    try {
      loading.stop.value = true;
      status.value = await invoke("stop_alist");
      message.value = "alist 已停止！";
    } catch (error) {
      message.value = `停止失败：${error}`;
    } finally {
      loading.stop.value = false;
    }
  }

  async function downloadAlist() {
    try {
      loading.download.value = true;
      const options = {
        proxyUrl: useProxy.value ? proxyUrl.value : null,
        proxyUsername: useProxy.value ? proxyUsername.value : null,
        proxyPassword: useProxy.value ? proxyPassword.value : null,
      };
      await invoke("download_and_extract_alist", options);
      message.value = "alist 下载并解压成功！";
    } catch (error) {
      message.value = `下载并解压失败：${error}`;
    } finally {
      loading.download.value = false;
    }
  }

  async function getAlistVersion() {
    try {
      loading.version.value = true;
      const versionData = await invoke<AlistVersionInfo>("get_alist_version");
      versionInfo.value = {
        version: versionData.version,
        web_version: versionData.web_version,
        build_date: versionData.built_at,
        commit_hash: versionData.commit_id,
        platform: versionData.go_version?.includes('linux') ? 'Linux' : 
                 versionData.go_version?.includes('windows') ? 'Windows' : 
                 versionData.go_version?.includes('darwin') ? 'macOS' : undefined,
        built_at: versionData.built_at,
        go_version: versionData.go_version,
        author: versionData.author,
        commit_id: versionData.commit_id
      };
      showVersionDialog.value = true;
    } catch (error) {
      message.value = `获取版本信息失败：${error}`;
      window.$notification?.error({
        content: `获取版本信息失败：${error}`,
        duration: 5000,
        keepAliveOnHover: true,
      });
    } finally {
      loading.version.value = false;
    }
  }

  async function deleteDataFolder() {
    try {
      await invoke("delete_data_folder");
      console.log("Data folder deleted successfully");
    } catch (error) {
      console.error("Failed to delete data folder:", error);
    }
  }

  return {
    startPolling,
    stopPolling,
    getAlistStatus,
    startAlist,
    stopAlist,
    downloadAlist,
    getAlistVersion,
    deleteDataFolder
  };
}

onMounted(async () => {
  try {
    api.value = await useAlistApi();
    api.value?.startPolling();
  } catch (error) {
    console.error('Failed to initialize API:', error);
  }
});

onUnmounted(() => {
  api.value?.stopPolling();
});
</script>

<template>
  <BaseLayout>
    <template #header>
      <h1>Status Page</h1>
      <p>这是状态页面</p>
    </template>

    <n-card class="status-card">
      <n-space vertical>
        <n-alert :type="status.running ? 'success' : 'error'">
          当前 alist 状态：{{ status.running ? "运行中" : "已停止" }}
          <template #icon>
            <n-icon :name="status.running ? 'checkmark-circle' : 'close-circle'" />
          </template>
        </n-alert>

        <n-descriptions label-placement="left" bordered>
          <n-descriptions-item label="进程 ID">
            {{ status.pid || "无" }}
          </n-descriptions-item>
          <n-descriptions-item label="状态信息">
            {{ message }}
          </n-descriptions-item>
          <n-descriptions-item label="版本信息" v-if="versionInfo">
            <n-space vertical>
              <n-tag type="info" size="small">核心版本: {{ versionInfo.version }}</n-tag>
              <n-tag type="info" size="small">Web版本: {{ versionInfo.web_version }}</n-tag>
              <n-tag v-if="versionInfo.build_date" type="info" size="small">
                构建日期: {{ versionInfo.build_date }}
              </n-tag>
              <n-tag v-if="versionInfo.commit_hash" type="info" size="small">
                Git提交: {{ versionInfo.commit_hash.slice(0, 7) }}
              </n-tag>
              <n-tag v-if="versionInfo.platform" type="info" size="small">
                平台: {{ versionInfo.platform }}
              </n-tag>
            </n-space>
          </n-descriptions-item>
        </n-descriptions>

        <n-space justify="center">
          <n-button-group>
            <n-button 
              @click="async () => await api?.getAlistStatus()" 
              type="primary" 
              :loading="loading.status.value"
            >
              刷新状态
            </n-button>
            <n-button 
              @click="async () => await api?.startAlist()"
              :disabled="status.running || loading.start.value"
              :loading="loading.start.value"
              v-if="!status.running"
              type="success"
            >
              启动 alist
            </n-button>
            <n-button 
              @click="async () => await api?.stopAlist()"
              :disabled="!status.running || loading.stop.value"
              :loading="loading.stop.value"
              v-if="status.running"
              type="error"
            >
              停止 alist
            </n-button>
          </n-button-group>
        </n-space>

        <n-space justify="center" class="additional-actions">
          <n-button @click="async () => await api?.getAlistVersion()" secondary :loading="loading.version.value">
            获取版本信息
          </n-button>
          <n-button @click="showOptions = true" secondary>
            可选参数
          </n-button>
          <n-button 
            @click="async () => await api?.downloadAlist()"
            :disabled="status.running || loading.download.value"
            :loading="loading.download.value"
            v-if="!status.running"
            secondary
          >
            下载 alist
          </n-button>
        </n-space>

        <Change_password class="change-password" />
      </n-space>
    </n-card>

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

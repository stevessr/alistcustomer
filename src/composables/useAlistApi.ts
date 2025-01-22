import { ref, nextTick, type Ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { useStatus } from "../views/status/status";
import type { AlistStatus, AlistVersionInfo, Metrics } from "@/types/alist";

export function useAlistApi() {
  const statusStore = useStatus<AlistStatus>();

  const loading = {
    status: ref(false),
    start: ref(false),
    stop: ref(false),
    download: ref(false),
    version: ref(false),
    metrics: ref(false)
  };

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
      statusStore.status.value.running = statusResult.running;
      statusStore.status.value.pid = statusResult.pid;
      statusStore.message = "状态获取成功！";
    } catch (error: unknown) {
      const err = error instanceof Error ? error : new Error(String(error));
      statusStore.message = `获取状态失败：${err.message}`;
    } finally {
      loading.status.value = false;
    }
  }

  async function startAlist() {
    try {
      loading.start.value = true;
      const startResult = await invoke<AlistStatus>("start_alist");
      statusStore.status.value.running = startResult.running;
      statusStore.status.value.pid = startResult.pid;
      statusStore.message = "alist 启动成功！";
    } catch (error: unknown) {
      const err = error instanceof Error ? error : new Error(String(error));
      statusStore.message = `启动失败：${err.message}`;
    } finally {
      loading.start.value = false;
    }
  }

  async function stopAlist() {
    try {
      loading.stop.value = true;
      const stopResult = await invoke<AlistStatus>("stop_alist");
      statusStore.status.value.running = stopResult.running;
      statusStore.status.value.pid = stopResult.pid;
      statusStore.message = "alist 已停止！";
    } catch (error: unknown) {
      const err = error instanceof Error ? error : new Error(String(error));
      statusStore.message = `停止失败：${err.message}`;
    } finally {
      loading.stop.value = false;
    }
  }

  async function downloadAlist() {
    try {
      loading.download.value = true;
      const options = {
        proxyUrl: statusStore.useProxy && statusStore.proxyUrl ? statusStore.proxyUrl : null,
        proxyUsername: statusStore.useProxy && statusStore.proxyUsername ? statusStore.proxyUsername : null,
        proxyPassword: statusStore.useProxy && statusStore.proxyPassword ? statusStore.proxyPassword : null,
      };
      await invoke("download_and_extract_alist", options);
      statusStore.message = "alist 下载并解压成功！";
    } catch (error: unknown) {
      const err = error instanceof Error ? error : new Error(String(error));
      statusStore.message = `下载并解压失败：${err.message}`;
    } finally {
      loading.download.value = false;
    }
  }

  async function getAlistVersion() {
    try {
      loading.version.value = true;
      const versionData = await invoke<AlistVersionInfo>("get_alist_version");
      statusStore.versionInfo = {
        version: versionData.version,
        web_version: versionData.web_version,
        build_date: versionData.built_at,
        commit_hash: versionData.commit_id,
        built_at: versionData.built_at,
        go_version: versionData.go_version,
        author: versionData.author,
        commit_id: versionData.commit_id
      };
      
      if (window.$message) {
        window.$message.success('版本信息更新成功！');
      }

      statusStore.showVersionDialog = false;
      await nextTick();
      statusStore.showVersionDialog = true;
    } catch (error: unknown) {
      const err = error instanceof Error ? error : new Error(String(error));
      statusStore.message = `获取版本信息失败：${err.message}`;
      if (window.$notification) {
        window.$notification.error({
          content: `获取版本信息失败：${err.message}`,
          duration: 5000,
          keepAliveOnHover: true,
        });
      }
    } finally {
      loading.version.value = false;
    }
  }

  async function deleteDataFolder() {
    try {
      await invoke("delete_data_folder");
      console.log("Data folder deleted successfully");
    } catch (error: unknown) {
      const err = error instanceof Error ? error : new Error(String(error));
      console.error("Failed to delete data folder:", err);
    }
  }

  async function getMetrics(): Promise<Metrics> {
    try {
      loading.metrics.value = true;
      const metrics = await invoke<Metrics>("get_alist_metrics");
      return {
        uptime: metrics.uptime,
        cpuUsage: metrics.cpuUsage,
        memoryUsage: metrics.memoryUsage
      };
    } catch (error: unknown) {
      const err = error instanceof Error ? error : new Error(String(error));
      console.error("Failed to get metrics:", err);
      throw err;
    } finally {
      loading.metrics.value = false;
    }
  }

  return {
    status: statusStore.status,
    message: statusStore.message,
    versionInfo: statusStore.versionInfo,
    loading,
    startPolling,
    stopPolling,
    getAlistStatus,
    startAlist,
    stopAlist,
    downloadAlist,
    getAlistVersion,
    deleteDataFolder,
    getMetrics
  };
}

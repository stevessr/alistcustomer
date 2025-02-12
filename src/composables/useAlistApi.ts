import { ref, nextTick, type Ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { useStatus } from "../views/status/status";
import type { AlistStatus, AlistVersionInfo, Metrics, ProcessMetrics } from "@/types/alist";

export function useAlistApi() {
  const statusStore = useStatus()

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
    }, 20000);
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
      
      // 【修复1】直接调用 get_alist_status 接口
      const statusResult = await invoke<ProcessMetrics>("get_alist_status"); // 确保AlistStatus与ProcessMetrics类型一致
      
      if (import.meta.env.MODE === 'development') {
        console.log(`[API] get_alist_status response:`, statusResult);
      }
  
      // 【修复2】简化验证逻辑
      const isValidResponse = statusResult && 
        typeof statusResult.running === 'boolean' &&
        (statusResult.pid === null || typeof statusResult.pid === 'string');
  
      if (!isValidResponse) {
        throw new Error("Invalid alist status response");
      }
  
      // 【修复3】直接使用接口返回的状态
      statusStore.status.value = {
        running: statusResult.running,
        pid: statusResult.pid || undefined // 转换为undefined保持类型一致
      };
      
      statusStore.message.value = "状态获取成功！";
      return statusResult;
    } catch (error: unknown) {
      const err = error instanceof Error ? error : new Error(String(error));
      statusStore.status.value = { running: false, pid: undefined };
      statusStore.message.value = `获取状态失败：${err.message}`;
      throw err; // 抛出错误供调用方处理
    } finally {
      loading.status.value = false;
    }
  }

  async function startAlist() {
    try {
      loading.start.value = true;
      if (import.meta.env.MODE === 'development') {
        console.log(`[API] Calling start_alist`);
      }
      const startResult = await invoke<AlistStatus>("start_alist");
    statusStore.status.value.running = startResult.running;
    statusStore.status.value.pid = startResult.pid;
    statusStore.message.value = "alist 启动成功！";
    // 启动后主动刷新状态
    await getAlistStatus();
    } catch (error: unknown) {
      const err = error instanceof Error ? error : new Error(String(error));
      statusStore.message.value = `启动失败：${err.message}`;
    } finally {
      loading.start.value = false;
    }
  }

  async function stopAlist() {
    try {
      loading.stop.value = true;
      if (import.meta.env.MODE === 'development') {
        console.log(`[API] Calling stop_alist`);
      }
      const stopResult = await invoke<AlistStatus>("stop_alist");
      statusStore.status.value.running = stopResult.running;
      statusStore.status.value.pid = stopResult.pid;
      statusStore.message.value = "alist 已停止！";
    } catch (error: unknown) {
      const err = error instanceof Error ? error : new Error(String(error));
      statusStore.message.value = `停止失败：${err.message}`;
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
      statusStore.message.value = "alist 下载并解压成功！";
    } catch (error: unknown) {
      const err = error instanceof Error ? error : new Error(String(error));
      statusStore.message.value = `下载并解压失败：${err.message}`;
    } finally {
      loading.download.value = false;
    }
  }

  async function getAlistVersion(): Promise<AlistVersionInfo> {
    try {
      loading.version.value = true;
      const versionData = await invoke<AlistVersionInfo>("get_alist_version");
      console.log(versionData);
      statusStore.versionInfo.value = versionData;
      statusStore.message.value = '版本信息更新成功！';
      return versionData;
    } catch (error: unknown) {
      const err = error instanceof Error ? error : new Error(String(error));
      statusStore.message.value = `获取版本信息失败：${err.message}`;
      throw err; // 抛出错误供组件层处理
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
      const processMetrics = await invoke<ProcessMetrics>("manage_alist_state");
      if (!processMetrics) {
        throw new Error("Failed to get process metrics");
      }
      return {
        uptime: 0, // TODO: Add uptime tracking
        cpuUsage: processMetrics.cpu_usage || 0,
        memoryUsage: processMetrics.memory_usage || 0
      };
    } catch (error: unknown) {
      const err = error instanceof Error ? error : new Error(String(error));
      console.error("Failed to get metrics:", err);
      throw new Error(`Failed to get metrics: ${err.message}`);
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

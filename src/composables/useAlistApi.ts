import { ref, nextTick } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { useStatus } from "../views/status/status";
import type { AlistStatus, AlistVersionInfo } from "@/types/alist";

export function useAlistApi() {
  const statusStore = useStatus();

  const loading = {
    status: ref(false),
    start: ref(false),
    stop: ref(false),
    download: ref(false),
    version: ref(false)
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
      statusStore.status = {
        running: statusResult.running,
        pid: statusResult.pid
      };
      statusStore.message = "状态获取成功！";
    } catch (error) {
      statusStore.message = `获取状态失败：${error}`;
    } finally {
      loading.status.value = false;
    }
  }

  async function startAlist() {
    try {
      loading.start.value = true;
      statusStore.status = await invoke("start_alist");
      statusStore.message = "alist 启动成功！";
    } catch (error) {
      statusStore.message = `启动失败：${error}`;
    } finally {
      loading.start.value = false;
    }
  }

  async function stopAlist() {
    try {
      loading.stop.value = true;
      statusStore.status = await invoke("stop_alist");
      statusStore.message = "alist 已停止！";
    } catch (error) {
      statusStore.message = `停止失败：${error}`;
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
    } catch (error) {
      statusStore.message = `下载并解压失败：${error}`;
    } finally {
      loading.download.value = false;
    }
  }

  async function getAlistVersion() {
    try {
      loading.version.value = true;
      const versionData = await invoke<AlistVersionInfo>("get_alist_version");
      // 强制创建新对象以触发响应式更新
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
      console.log('版本信息已更新:', statusStore.versionInfo);
      
      // 显示成功提示
      window.$message?.success('版本信息更新成功！');

      // 先关闭再打开对话框确保父组件更新
      statusStore.showVersionDialog = false;
      await nextTick();
      statusStore.showVersionDialog = true;
    } catch (error) {
      statusStore.message = `获取版本信息失败：${error}`;
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
    deleteDataFolder
  };
}

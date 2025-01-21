import { invoke } from "@tauri-apps/api/core";
import { Store } from "@tauri-apps/plugin-store";
import type { AlistStatus, AlistVersionInfo } from "../../types/alist";
import { useStatus } from "./status";

export const useAlistApi = async () => {
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
      window.$notification?.error({
        content: `获取状态失败：${error}`,
        duration: 5000,
        keepAliveOnHover: true,
      });
    }
  }

  async function startAlist() {
    try {
      status.value = await invoke("start_alist");
      message.value = "alist 启动成功！";
      window.$notification?.success({
        content: "alist 启动成功！",
        duration: 5000,
        keepAliveOnHover: true,
      });
    } catch (error) {
      message.value = `启动失败：${error}`;
      window.$notification?.error({
        content: `启动失败：${error}`,
        duration: 5000,
        keepAliveOnHover: true,
      });
    }
  }

  async function stopAlist() {
    try {
      status.value = await invoke("stop_alist");
      message.value = "alist 已停止！";
      window.$notification?.success({
        content: "alist 已停止！",
        duration: 5000,
        keepAliveOnHover: true,
      });
    } catch (error) {
      message.value = `停止失败：${error}`;
      window.$notification?.error({
        content: `停止失败：${error}`,
        duration: 5000,
        keepAliveOnHover: true,
      });
    }
  }

  async function downloadAlist() {
    try {
      const options = {
        proxyUrl: useProxy.value ? proxyUrl.value : null,
        proxyUsername: useProxy.value ? proxyUsername.value : null,
        proxyPassword: useProxy.value ? proxyPassword.value : null,
      };
      await invoke("download_and_extract_alist", options);
      message.value = "alist 下载并解压成功！";
      window.$notification?.success({
        content: "alist 下载并解压成功！",
        duration: 5000,
        keepAliveOnHover: true,
      });
    } catch (error) {
      message.value = `下载并解压失败：${error}`;
      window.$notification?.error({
        content: `下载并解压失败：${error}`,
        duration: 5000,
        keepAliveOnHover: true,
      });
    }
  }

  async function getAlistVersion() {
    try {
      versionInfo.value = await invoke<AlistVersionInfo>("get_alist_version");
    } catch (error) {
      message.value = `获取版本信息失败：${error}`;
      window.$notification?.error({
        content: `获取版本信息失败：${error}`,
        duration: 5000,
        keepAliveOnHover: true,
      });
    }
  }

  async function deleteDataFolder() {
    try {
      await invoke("delete_data_folder");
      message.value = "数据文件夹删除成功！";
      window.$notification?.success({
        content: "数据文件夹删除成功！",
        duration: 5000,
        keepAliveOnHover: true,
      });
    } catch (error) {
      message.value = `删除数据文件夹失败：${error}`;
      window.$notification?.error({
        content: `删除数据文件夹失败：${error}`,
        duration: 5000,
        keepAliveOnHover: true,
      });
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
};

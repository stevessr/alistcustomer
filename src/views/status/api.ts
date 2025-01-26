import { invoke } from "@tauri-apps/api/core";
import { Store } from "@tauri-apps/plugin-store";
import type { AlistStatus, AlistVersionInfo } from "../../types/alist";
import { useStatus } from "./status";
import type { Ref } from "vue";
import type { NotificationApi } from "naive-ui";

export function createStartPolling(status: Ref<AlistStatus>, message: Ref<string>) {
  let pollTimer: number | null = null;

  return function() {
    const stop = createStopPolling();
    stop();
    pollTimer = window.setInterval(async () => {
      const getStatus = await createGetAlistStatus(status, message);
      await getStatus();
    }, 2000);
  };
}

export function createStopPolling() {
  let pollTimer: number | null = null;

  return function() {
    if (pollTimer !== null) {
      window.clearInterval(pollTimer);
      pollTimer = null;
    }
  };
}

export function createGetAlistStatus(status: Ref<AlistStatus>, message: Ref<string>) {
  return async function() {
    const alistStatusStore = await Store.load("alist-status.store");
    try {
      await invoke("manage_alist_state");
      const statusResult = await invoke<AlistStatus>("get_alist_status");
      if (!statusResult) {
        throw new Error("Failed to get alist status");
      }
      await alistStatusStore.set("status", statusResult);
      const storedStatus = await alistStatusStore.get<AlistStatus>("status");
      if (storedStatus) {
        status.value = storedStatus;
      }
      message.value = "状态获取成功！";
    } catch (error) {
      message.value = `获取状态失败：${error}`;
      (window.$notification as NotificationApi)?.create({
        content: `获取状态失败：${error}`,
        duration: 5000,
        keepAliveOnHover: true,
        type: 'error'
      });
    }
  };
}

export function createStartAlist(status: Ref<AlistStatus>, message: Ref<string>) {
  return async function() {
    try {
      const result = await invoke<AlistStatus>("start_alist");
      status.value = result;
      message.value = "alist 启动成功！";
      (window.$notification as NotificationApi)?.create({
        content: "alist 启动成功！",
        duration: 5000,
        keepAliveOnHover: true,
        type: 'success'
      });
    } catch (error) {
      message.value = `启动失败：${error}`;
      (window.$notification as NotificationApi)?.create({
        content: `启动失败：${error}`,
        duration: 5000,
        keepAliveOnHover: true,
        type: 'error'
      });
    }
  };
}

export function createStopAlist(status: Ref<AlistStatus>, message: Ref<string>) {
  return async function() {
    try {
      const result = await invoke<AlistStatus>("stop_alist");
      status.value = result;
      message.value = "alist 已停止！";
      (window.$notification as NotificationApi)?.create({
        content: "alist 已停止！",
        duration: 5000,
        keepAliveOnHover: true,
        type: 'success'
      });
    } catch (error) {
      message.value = `停止失败：${error}`;
      (window.$notification as NotificationApi)?.create({
        content: `停止失败：${error}`,
        duration: 5000,
        keepAliveOnHover: true,
        type: 'error'
      });
    }
  };
}

export function createDownloadAlist(
  proxyUrl: Ref<string>,
  proxyUsername: Ref<string>,
  proxyPassword: Ref<string>,
  useProxy: Ref<boolean>,
  message: Ref<string>
) {
  return async function() {
    try {
      const options = {
        proxyUrl: useProxy.value ? proxyUrl.value : null,
        proxyUsername: useProxy.value ? proxyUsername.value : null,
        proxyPassword: useProxy.value ? proxyPassword.value : null,
      };
      await invoke("download_and_extract_alist", options);
      message.value = "alist 下载并解压成功！";
      (window.$notification as NotificationApi)?.create({
        content: "alist 下载并解压成功！",
        duration: 5000,
        keepAliveOnHover: true,
        type: 'success'
      });
    } catch (error) {
      message.value = `下载并解压失败：${error}`;
      (window.$notification as NotificationApi)?.create({
        content: `下载并解压失败：${error}`,
        duration: 5000,
        keepAliveOnHover: true,
        type: 'error'
      });
    }
  };
}

export function createGetAlistVersion(versionInfo: Ref<AlistVersionInfo | null>, message: Ref<string>) {
  return async function() {
    try {
      const result = await invoke<AlistVersionInfo>("get_alist_version");
      versionInfo.value = result;
      return result;
    } catch (error) {
      message.value = `获取版本信息失败：${error}`;
      (window.$notification as NotificationApi)?.create({
        content: `获取版本信息失败：${error}`,
        duration: 5000,
        keepAliveOnHover: true,
        type: 'error'
      });
      return null;
    }
  };
}

export function createDeleteDataFolder(message: Ref<string>) {
  return async function() {
    try {
      await invoke("delete_data_folder");
      message.value = "数据文件夹删除成功！";
      (window.$notification as NotificationApi)?.create({
        content: "数据文件夹删除成功！",
        duration: 5000,
        keepAliveOnHover: true,
        type: 'success'
      });
    } catch (error) {
      message.value = `删除数据文件夹失败：${error}`;
      (window.$notification as NotificationApi)?.create({
        content: `删除数据文件夹失败：${error}`,
        duration: 5000,
        keepAliveOnHover: true,
        type: 'error'
      });
    }
  };
}

export const useAlistApi = () => {
  const {
    status,
    message,
    proxyUrl,
    proxyUsername,
    proxyPassword,
    useProxy,
    versionInfo
  } = useStatus();

  return {
    startPolling: createStartPolling(status, message),
    stopPolling: createStopPolling(),
    getAlistStatus: createGetAlistStatus(status, message),
    startAlist: createStartAlist(status, message),
    stopAlist: createStopAlist(status, message),
    downloadAlist: createDownloadAlist(proxyUrl, proxyUsername, proxyPassword, useProxy, message),
    getAlistVersion: createGetAlistVersion(versionInfo, message),
    deleteDataFolder: createDeleteDataFolder(message)
  };
};

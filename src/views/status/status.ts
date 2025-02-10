import { ref, reactive } from "vue";
import type { Ref } from "vue";
import type { AlistStatus, AlistVersionInfo } from "../../types/alist";
import { invoke } from '@tauri-apps/api/core';

export const useStatus = () => {
  const status = ref<AlistStatus>({ running: false, pid: null });
  const message = ref("");
  const loading = ref(false);
  const error = ref(false);
  const pollInterval = ref(2000);
  const proxyUrl = ref("");
  const proxyUsername = ref("");
  const proxyPassword = ref("");
  const showOptions = ref(false);
  const useProxy = ref(false);
  const showVersionDialog = ref(false);
  const versionInfo = ref<AlistVersionInfo | null>(null);

  const checkSystemStatus = async () => {
    try {
      return await invoke<AlistStatus>('get_alist_status');
    } catch (error) {
      console.error('Failed to check alist status:', error);
      return { running: false, pid: null };
    }
  };

  return {
    status,
    message,
    loading,
    error,
    pollInterval,
    proxyUrl,
    proxyUsername,
    proxyPassword,
    useProxy,
    versionInfo,
    showOptions,
    showVersionDialog,
    checkSystemStatus
  };
};

import { ref, reactive } from "vue";
import type { Ref } from "vue";
import type { AlistStatus, AlistVersionInfo } from "../../types/alist";

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
    // Simulate an async operation that returns a status
    return new Promise<boolean>((resolve) => {
      setTimeout(() => {
        resolve(Math.random() > 0.5);
      }, 1000);
    });
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
    versionInfo
  };
};

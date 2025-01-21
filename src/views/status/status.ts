import { ref } from "vue";
import type { AlistStatus, AlistVersionInfo } from "@/types/alist";

export const useStatus = () => {
  const status = ref<AlistStatus>({ running: false, pid: undefined });
  const message = ref("");
  const loading = {
    status: ref(false),
    start: ref(false),
    stop: ref(false),
    download: ref(false),
    version: ref(false)
  };
  const error = ref(false);
  const pollInterval = ref(2000);
  const proxyUrl = ref("");
  const proxyUsername = ref("");
  const proxyPassword = ref("");
  const showOptions = ref(false);
  const useProxy = ref(false);
  const showVersionDialog = ref(false);
  const versionInfo = ref<AlistVersionInfo | null>(null);

  return {
    status,
    message,
    loading,
    error,
    pollInterval,
    proxyUrl,
    proxyUsername,
    proxyPassword,
    showOptions,
    useProxy,
    showVersionDialog,
    versionInfo
  };
};

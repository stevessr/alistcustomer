import { ref, reactive } from "vue";
import type { AlistStatus, AlistVersionInfo } from "../../types/alist";

export const useStatus = () => {
  const status = ref<AlistStatus>({ running: false, pid: undefined });
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

  return reactive({
    status,
    message,
    loading: loading,
    error,
    pollInterval,
    proxyUrl,
    proxyUsername,
    proxyPassword,
    showOptions,
    useProxy,
    showVersionDialog,
    versionInfo
  });
};

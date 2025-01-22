import type { Ref } from 'vue';

export interface AlistStatus {
  running: boolean;
  pid: number | undefined;
}

export type AlistStatusRef = Ref<AlistStatus>;

export interface AlistVersionInfo {
  version: string;
  web_version: string;
  build_date?: string;
  commit_hash?: string;
  platform?: string;
  built_at?: string;
  go_version?: string;
  author?: string;
  commit_id?: string;
}

export interface Metrics {
  uptime: number;
  cpuUsage: number;
  memoryUsage: number;
}

export interface StatusHistoryRecord {
  type: 'success' | 'error' | 'info' | 'warning';
  title: string;
  content: string;
  time: string;
  status: boolean;
}

export interface AlistApi {
  startPolling(): void;
  stopPolling(): void;
  getAlistStatus(): Promise<AlistStatus>;
  startAlist(): Promise<void>;
  stopAlist(): Promise<void>;
  downloadAlist(): Promise<void>;
  getAlistVersion(): Promise<AlistVersionInfo>;
  deleteDataFolder(): Promise<void>;
  getMetrics(): Promise<Metrics>;
}

declare global {
  interface Window {
    $notification: {
      error: (options: {
        content: string;
        duration?: number;
        keepAliveOnHover?: boolean;
      }) => void;
    };
  }
}

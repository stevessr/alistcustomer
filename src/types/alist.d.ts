import type { Ref } from 'vue';

export interface AlistStatus {
  running: boolean;
  pid?: number | undefined;
}

export interface AlistMetrics {
  cpu_usage: number
  memory_usage: number
  uptime: number
  threads: number
}

export type AlistStatusRef = Ref<AlistStatus>;

export interface AlistVersionInfo {
  version: string;
  web_version: string;
  built_at?: string;
  go_version?: string;
  author?: string;
  commit_id?: string;
  platform?: string;
}

export interface ProcessMetrics {
  pid: string | null;
  running: boolean;
  cpu_usage: number | null;
  memory_usage: number | null;
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

export interface AlistStatusResponse {
  status: AlistStatus;
  metrics: Metrics;
  version: AlistVersionInfo;
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

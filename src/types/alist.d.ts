export interface AlistStatus {
  running: boolean;
  pid: number | undefined;
}

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

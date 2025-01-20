declare interface AlistStatus {
  running: boolean;
  pid: number | null;
}

declare interface AlistVersionInfo {
  version: string;
  web_version: string;
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

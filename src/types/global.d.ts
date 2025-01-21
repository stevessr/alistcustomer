declare global {
  interface Window {
    $message: import('naive-ui').MessageApi
  }
}

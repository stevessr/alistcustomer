<script setup lang="ts">
import { defineProps, toRefs } from 'vue';

interface Config {
  section_colors?: {
    ftp?: string;
  };
  s3?: {
    enable?: boolean;
    port?: number;
    ssl?: boolean;
  };
  sftp?: {
    enable?: boolean;
    listen?: string;
  };
  ftp?: {
    enable?: boolean;
    listen?: string;
    find_pasv_port_attempts?: number;
    active_transfer_port_non_20?: boolean;
    idle_timeout?: number;
    connection_timeout?: number;
    disable_active_mode?: boolean;
    default_transfer_binary?: boolean;
    enable_active_conn_ip_check?: boolean;
    enable_pasv_conn_ip_check?: boolean;
  };
}

const props = withDefaults(defineProps<{
  config?: Config;
}>(), {
  config: () => ({
    section_colors: {
      ftp: ''
    },
    s3: {
      enable: false,
      port: 0,
      ssl: false
    },
    sftp: {
      enable: false,
      listen: ''
    },
    ftp: {
      enable: false,
      listen: '',
      find_pasv_port_attempts: 0,
      active_transfer_port_non_20: false,
      idle_timeout: 0,
      connection_timeout: 0,
      disable_active_mode: false,
      default_transfer_binary: false,
      enable_active_conn_ip_check: false,
      enable_pasv_conn_ip_check: false
    }
  })
});

const { config } = toRefs(props);
</script>

<template>
  <div class="config-section">
    <h2>FTP配置</h2>
    <n-form>
      <n-form-item label="启用FTP:" path="ftpEnable">
        <n-switch :checked="config.ftp?.enable ?? false" @update:checked="(val: boolean) => config.ftp!.enable = val" />
      </n-form-item>
      
      <n-form-item label="监听地址:" path="ftpListen">
        <n-input
          :value="config.ftp?.listen ?? ''" @update:value="(val: string) => config.ftp!.listen = val"
          placeholder="输入监听地址"
        />
      </n-form-item>
      
      <n-form-item label="查找PASV端口尝试次数:" path="ftpPasvAttempts">
        <n-input-number
          :value="config.ftp?.find_pasv_port_attempts ?? 0" @update:value="(val: number) => config.ftp!.find_pasv_port_attempts = val"
          :min="0"
        />
      </n-form-item>
      
      <!-- Other fields will follow the same pattern -->
    </n-form>
  </div>
</template>

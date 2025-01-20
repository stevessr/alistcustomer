<script setup lang="ts">
import { defineProps } from 'vue';

defineProps<{
  config: {
    ftp: {
      enable: boolean;
      listen: string;
      find_pasv_port_attempts: number;
      active_transfer_port_non_20: boolean;
      idle_timeout: number;
      connection_timeout: number;
      disable_active_mode: boolean;
      default_transfer_binary: boolean;
      enable_active_conn_ip_check: boolean;
      enable_pasv_conn_ip_check: boolean;
    };
  };
}>();
</script>

<template>
  <div class="config-section">
    <h2>FTP Configuration</h2>
    <n-form>
      <n-form-item label="Enable FTP:" path="ftpEnable">
        <n-switch v-model:checked="config.ftp.enable" />
      </n-form-item>
      
      <n-form-item label="Listen Address:" path="ftpListen">
        <n-input
          v-model:value="config.ftp.listen"
          placeholder="Enter listen address"
        />
      </n-form-item>
      
      <n-form-item label="Find PASV Port Attempts:" path="ftpPasvAttempts">
        <n-input-number
          v-model:value="config.ftp.find_pasv_port_attempts"
          :min="0"
        />
      </n-form-item>
      
      <!-- Other fields will follow the same pattern -->
    </n-form>
  </div>
</template>

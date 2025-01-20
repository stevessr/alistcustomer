<script setup lang="ts">
import { defineProps } from 'vue';

defineProps<{
  config: {
    log: {
      enable: boolean;
      name: string;
      max_size: number;
      max_backups: number;
      max_age: number;
      compress: boolean;
    };
  };
}>();
</script>

<template>
  <div class="config-section">
    <h2>Log Configuration</h2>
    <n-form>
      <n-form-item label="Enable Logging:" path="logEnable">
        <n-switch v-model:checked="config.log.enable" />
      </n-form-item>

      <n-form-item label="Log Name:" path="logName">
        <n-input
          v-model:value="config.log.name"
          placeholder="Enter log name"
        />
      </n-form-item>

      <n-form-item label="Max Size (MB):" path="logMaxSize">
        <n-input-number
          v-model:value="config.log.max_size"
          :min="0"
        />
      </n-form-item>

      <n-form-item label="Max Backups:" path="logMaxBackups">
        <n-input-number
          v-model:value="config.log.max_backups"
          :min="0"
        />
      </n-form-item>

      <n-form-item label="Max Age (days):" path="logMaxAge">
        <n-input-number
          v-model:value="config.log.max_age"
          :min="0"
        />
      </n-form-item>

      <n-form-item label="Compress Logs:" path="logCompress">
        <n-switch v-model:checked="config.log.compress" />
      </n-form-item>
    </n-form>
  </div>
</template>

<script setup lang="ts">
import { defineProps } from 'vue';

defineProps<{
  config: {
    s3: {
      enable: boolean;
      port: number;
      ssl: boolean;
    };
  };
}>();
</script>

<template>
  <div class="config-section">
    <h2>S3 Configuration</h2>
    <n-form>
      <n-form-item label="Enable S3:" path="s3Enable">
        <n-switch v-model:checked="config.s3.enable" />
      </n-form-item>

      <n-form-item label="Port:" path="s3Port">
        <n-input-number
          v-model:value="config.s3.port"
          :min="0"
          :max="65535"
        />
      </n-form-item>

      <n-form-item label="SSL:" path="s3Ssl">
        <n-switch v-model:checked="config.s3.ssl" />
      </n-form-item>
    </n-form>
  </div>
</template>

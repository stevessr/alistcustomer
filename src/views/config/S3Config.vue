<script setup lang="ts">
import { defineProps, computed } from 'vue';

interface S3Config {
  enable: boolean;
  port: number;
  ssl: boolean;
}

interface Config {
  s3: S3Config;
}

const props = withDefaults(defineProps<{
  config?: Config;
}>(), {
  config: () => ({
    s3: {
      enable: false,
      port: 9000,
      ssl: false
    }
  })
});

const s3Config = computed(() => props.config.s3);
</script>

<template>
  <div class="config-section">
    <h2>S3存储配置</h2>
    <n-form>
      <n-form-item label="启用S3:" path="s3Enable">
        <n-switch v-model:checked="s3Config.enable" />
      </n-form-item>

      <n-form-item label="端口:" path="s3Port">
        <n-input-number
          v-model:value="s3Config.port"
          :min="0"
          :max="65535"
        />
      </n-form-item>

      <n-form-item label="SSL:" path="s3Ssl">
        <n-switch v-model:checked="s3Config.ssl" />
      </n-form-item>
    </n-form>
  </div>
</template>

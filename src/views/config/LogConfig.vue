<script setup lang="ts">
import { defineProps, withDefaults, computed } from 'vue';

interface LogConfig {
  enable: boolean;
  name: string;
  max_size: number;
  max_backups: number;
  max_age: number;
  compress: boolean;
}

interface Config {
  log: LogConfig;
}

const props = withDefaults(defineProps<{
  config?: Config;
}>(), {
  config: () => ({
    log: {
      enable: false,
      name: '',
      max_size: 100,
      max_backups: 5,
      max_age: 30,
      compress: false
    }
  })
});

const logEnable = computed({
  get: () => props.config?.log?.enable ?? false,
  set: (value) => {
    if (props.config?.log) {
      props.config.log.enable = value;
    }
  }
});

const logName = computed({
  get: () => props.config?.log?.name ?? '',
  set: (value) => {
    if (props.config?.log) {
      props.config.log.name = value;
    }
  }
});

const logMaxSize = computed({
  get: () => props.config?.log?.max_size ?? 100,
  set: (value) => {
    if (props.config?.log) {
      props.config.log.max_size = value;
    }
  }
});

const logMaxBackups = computed({
  get: () => props.config?.log?.max_backups ?? 5,
  set: (value) => {
    if (props.config?.log) {
      props.config.log.max_backups = value;
    }
  }
});

const logMaxAge = computed({
  get: () => props.config?.log?.max_age ?? 30,
  set: (value) => {
    if (props.config?.log) {
      props.config.log.max_age = value;
    }
  }
});

const logCompress = computed({
  get: () => props.config?.log?.compress ?? false,
  set: (value) => {
    if (props.config?.log) {
      props.config.log.compress = value;
    }
  }
});
</script>

<template>
  <div class="config-section">
    <h2>日志配置</h2>
    <n-form>
      <n-form-item label="启用日志:" path="logEnable">
        <n-switch v-model:checked="logEnable" />
      </n-form-item>

      <n-form-item label="日志名称:" path="logName">
        <n-input
          v-model:value="logName"
          placeholder="输入日志名称"
        />
      </n-form-item>

      <n-form-item label="最大大小(MB):" path="logMaxSize">
        <n-input-number
          v-model:value="logMaxSize"
          :min="0"
        />
      </n-form-item>

      <n-form-item label="最大备份数:" path="logMaxBackups">
        <n-input-number
          v-model:value="logMaxBackups"
          :min="0"
        />
      </n-form-item>

      <n-form-item label="最大保留天数:" path="logMaxAge">
        <n-input-number
          v-model:value="logMaxAge"
          :min="0"
        />
      </n-form-item>

      <n-form-item label="压缩日志:" path="logCompress">
        <n-switch v-model:checked="logCompress" />
      </n-form-item>
    </n-form>
  </div>
</template>

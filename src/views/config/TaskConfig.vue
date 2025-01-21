<script setup lang="ts">
import { defineProps, withDefaults, computed } from 'vue';

interface TaskConfig {
  workers: number;
  max_retry: number;
  task_persistant: boolean;
}

interface Config {
  tasks: {
    download: TaskConfig;
    transfer: TaskConfig;
    upload: TaskConfig;
    copy: TaskConfig;
  };
}

const props = withDefaults(defineProps<{
  config?: Config;
}>(), {
  config: () => ({
    tasks: {
      download: {
        workers: 0,
        max_retry: 0,
        task_persistant: false
      },
      transfer: {
        workers: 0,
        max_retry: 0,
        task_persistant: false
      },
      upload: {
        workers: 0,
        max_retry: 0,
        task_persistant: false
      },
      copy: {
        workers: 0,
        max_retry: 0,
        task_persistant: false
      }
    }
  })
});

const downloadConfig = computed(() => props.config?.tasks?.download);
</script>

<template>
  <div class="config-section">
    <h2>任务配置</h2>
    <n-form>
      <div>
        <h3>下载</h3>
      <n-form-item label="工作线程数:" path="downloadWorkers">
          <n-input-number
            v-if="downloadConfig"
            v-model:value="downloadConfig.workers"
            :min="0"
          />
          <n-alert v-else type="error">
            配置加载失败，请刷新页面或联系管理员
          </n-alert>
        </n-form-item>

        <n-form-item label="最大重试次数:" path="downloadMaxRetry">
          <n-input-number
            v-if="downloadConfig"
            v-model:value="downloadConfig.max_retry"
            :min="0"
          />
          <n-alert v-else type="error">
            配置加载失败，请刷新页面或联系管理员
          </n-alert>
        </n-form-item>

        <n-form-item label="任务持久化:" path="downloadTaskPersistant">
          <n-switch
            v-if="downloadConfig"
            v-model:checked="downloadConfig.task_persistant"
          />
          <n-alert v-else type="error">
            配置加载失败，请刷新页面或联系管理员
          </n-alert>
        </n-form-item>
      </div>

      <!-- Repeat similar structure for transfer, upload, and copy tasks -->
    </n-form>
  </div>
</template>

<script setup lang="ts">
import { defineProps } from 'vue';

defineProps<{
  config: {
    tasks: {
      download: {
        workers: number;
        max_retry: number;
        task_persistant: boolean;
      };
      transfer: {
        workers: number;
        max_retry: number;
        task_persistant: boolean;
      };
      upload: {
        workers: number;
        max_retry: number;
        task_persistant: boolean;
      };
      copy: {
        workers: number;
        max_retry: number;
        task_persistant: boolean;
      };
    };
  };
}>();
</script>

<template>
  <div class="config-section">
    <h2>Task Configuration</h2>
    <n-form>
      <div>
        <h3>Download</h3>
        <n-form-item label="Workers:" path="downloadWorkers">
          <n-input-number
            v-model:value="config.tasks.download.workers"
            :min="0"
          />
        </n-form-item>

        <n-form-item label="Max Retry:" path="downloadMaxRetry">
          <n-input-number
            v-model:value="config.tasks.download.max_retry"
            :min="0"
          />
        </n-form-item>

        <n-form-item label="Task Persistant:" path="downloadTaskPersistant">
          <n-switch v-model:checked="config.tasks.download.task_persistant" />
        </n-form-item>
      </div>

      <!-- Repeat similar structure for transfer, upload, and copy tasks -->
    </n-form>
  </div>
</template>

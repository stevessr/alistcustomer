<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core';

const emit = defineEmits(['save-config']);

const saveConfig = async (config: any) => {
  try {
    await invoke('write_config', { config });
    emit('save-config', true);
  } catch (error) {
    emit('save-config', false, error);
  }
};
</script>

<template>
  <div class="config-actions-side">
    <n-button 
      type="submit" 
      class="btn btn-primary"
      @click="$emit('save-config')"
    >
      保存配置
    </n-button>
  </div>
</template>

<style scoped>
.config-actions-side {
  margin-top: 24px;
  display: flex;
  justify-content: flex-end;
}
</style>

<script setup lang="ts">
import { NButton } from 'naive-ui';
import { useMessage } from 'naive-ui';

const message = useMessage();
const emit = defineEmits(['save-config']);

const handleSave = () => {
  try {
    emit('save-config');
    message.success('配置保存成功');
  } catch (error) {
    message.error(`配置保存失败: ${error}`);
  }
};
</script>

<template>
  <div class="config-actions-side">
    <n-button 
      type="primary"
      size="large"
      @click="handleSave"
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
  z-index: 1;
  position: relative;
}

.n-button {
  cursor: pointer;
  pointer-events: all;
}
</style>

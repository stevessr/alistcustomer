<script setup lang="ts">
import { NButton } from "naive-ui";
import { useMessage } from "naive-ui";

const message = useMessage();
const emit = defineEmits<{
  (e: 'save-config'): void
  (e: 'preview-config', config: unknown): void // 使用 unknown 替代 any 更安全
}>()

// 正确解构 props
const { config } = defineProps<{
  config?: unknown; // 使用 unknown 类型
}>();

const handleSave = () => {
  try {
    if (!config) {
      throw new Error('配置不能为空');
    }
    emit("save-config");
  } catch (error) {
    message.error(`保存失败: ${error instanceof Error ? error.message : '未知错误'}`);
    console.error(error);
  }
};

</script>

<template>
  <div class="config-actions-side">
    <n-button
      type="info"
      size="large"
      @click="emit('preview-config')"
      style="margin-right: 12px"
    >
      预览配置
    </n-button>
    <n-button type="primary" size="large" @click="handleSave">
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

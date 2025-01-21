<script setup lang="ts">
const emit = defineEmits(['save-config']);
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

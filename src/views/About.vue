<template>
  <BaseLayout>
    <n-card title="这是什么" class="about-card">
      写一下，测试一下自己而已
    </n-card>
  </BaseLayout>
</template>

<script setup lang="ts">
import BaseLayout from './components/BaseLayout.vue';
</script>

<style scoped>
.about-card {
  max-width: 800px;
  margin: 0 auto;
  padding: 2rem;
}

@media (max-width: 768px) {
  .about-card {
    padding: 1rem;
  }
}
</style>

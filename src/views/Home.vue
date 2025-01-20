<template>
  <BaseLayout>
    <template #header>
      <h1>Home Page</h1>
    </template>
    
    <n-card class="home-card">
      <p>Welcome to the AList Customer Portal</p>
    </n-card>
  </BaseLayout>
</template>

<script setup lang="ts">
import BaseLayout from './components/BaseLayout.vue';
</script>

<style scoped>
.home-card {
  max-width: 800px;
  margin: 0 auto;
  padding: 2rem;
  text-align: center;
}

@media (max-width: 768px) {
  .home-card {
    padding: 1rem;
  }
}
</style>

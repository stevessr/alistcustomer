<template>
  <BaseLayout>
    <template #header>
      <h1>Change Password</h1>
    </template>
    
    <change-password />
  </BaseLayout>
</template>

<script setup lang="ts">
import BaseLayout from "./components/BaseLayout.vue";
import ChangePassword from "./components/change_password.vue";
</script>

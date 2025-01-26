<script setup lang="ts">
import { NNotificationProvider, NMessageProvider } from 'naive-ui'
//import { useI18n } from 'vue-i18n'
import MENU from './com/menu.vue'

//const { t } = useI18n()
</script>

<template>
  <MENU />
  <n-notification-provider>
    <n-message-provider>
      <router-view></router-view>
    </n-message-provider>
  </n-notification-provider>
</template>

<style scoped>
/* Logo styles converted to Tailwind classes in template */
</style>

<style>
@import './assets/styles/base.css';
@import './assets/tailwind.scss';
</style>

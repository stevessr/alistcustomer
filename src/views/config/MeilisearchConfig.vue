<script setup lang="ts">
import { defineProps } from 'vue';

defineProps<{
  config: {
    meilisearch: {
      host: string;
      api_key: string;
      index_prefix: string;
    };
  };
}>();
</script>

<template>
  <div class="config-section">
    <h2>Meilisearch Configuration</h2>
    <n-form>
      <n-form-item label="Host:" path="meilisearchHost">
        <n-input
          v-model:value="config.meilisearch.host"
          placeholder="Enter Meilisearch host"
        />
      </n-form-item>

      <n-form-item label="API Key:" path="meilisearchApiKey">
        <n-input
          v-model:value="config.meilisearch.api_key"
          placeholder="Enter API key"
          type="password"
          show-password-on="click"
        />
      </n-form-item>

      <n-form-item label="Index Prefix:" path="meilisearchIndexPrefix">
        <n-input
          v-model:value="config.meilisearch.index_prefix"
          placeholder="Enter index prefix"
        />
      </n-form-item>
    </n-form>
  </div>
</template>

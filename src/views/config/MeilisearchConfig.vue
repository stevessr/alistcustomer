<script setup lang="ts">
import { defineProps, computed } from 'vue';

interface MeilisearchConfig {
  host: string;
  api_key: string;
  index_prefix: string;
}

const props = defineProps<{
  config?: {
    meilisearch?: MeilisearchConfig;
  };
}>();

const defaultConfig = {
  meilisearch: {
    host: '',
    api_key: '',
    index_prefix: ''
  }
};

const host = computed({
  get: () => props.config?.meilisearch?.host ?? defaultConfig.meilisearch.host,
  set: (value) => {
    if (props.config?.meilisearch) {
      props.config.meilisearch.host = value;
    }
  }
});

const apiKey = computed({
  get: () => props.config?.meilisearch?.api_key ?? defaultConfig.meilisearch.api_key,
  set: (value) => {
    if (props.config?.meilisearch) {
      props.config.meilisearch.api_key = value;
    }
  }
});

const indexPrefix = computed({
  get: () => props.config?.meilisearch?.index_prefix ?? defaultConfig.meilisearch.index_prefix,
  set: (value) => {
    if (props.config?.meilisearch) {
      props.config.meilisearch.index_prefix = value;
    }
  }
});
</script>

<template>
  <div class="config-section">
    <h2>Meilisearch配置</h2>
    <n-form>
      <n-form-item label="主机:" path="meilisearchHost">
        <n-input
          v-model:value="host"
          placeholder="输入Meilisearch主机地址"
        />
      </n-form-item>

      <n-form-item label="API密钥:" path="meilisearchApiKey">
        <n-input
          v-model:value="apiKey"
          placeholder="输入API密钥"
          type="password"
          show-password-on="click"
        />
      </n-form-item>

      <n-form-item label="索引前缀:" path="meilisearchIndexPrefix">
        <n-input
          v-model:value="indexPrefix"
          placeholder="输入索引前缀"
        />
      </n-form-item>
    </n-form>
  </div>
</template>

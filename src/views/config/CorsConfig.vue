<script setup lang="ts">
import { defineProps, computed } from 'vue';
import { NInput, NForm, NFormItem } from 'naive-ui';

const props = withDefaults(defineProps<{
  config?: {
    cors?: {
      allow_origins?: string[];
      allow_methods?: string[];
      allow_headers?: string[];
    };
  };
}>(), {
  config: () => ({
    cors: {
      allow_origins: [],
      allow_methods: [],
      allow_headers: []
    }
  })
});

const allowOrigins = computed({
  get() {
    return props.config?.cors?.allow_origins?.join(',') || '';
  },
  set(value) {
    if (props.config?.cors) {
      props.config.cors.allow_origins = value.split(',');
    }
  }
});

const allowMethods = computed({
  get() {
    return props.config?.cors?.allow_methods?.join(',') || '';
  },
  set(value) {
    if (props.config?.cors) {
      props.config.cors.allow_methods = value.split(',');
    }
  }
});

const allowHeaders = computed({
  get() {
    return props.config?.cors?.allow_headers?.join(',') || '';
  },
  set(value) {
    if (props.config?.cors) {
      props.config.cors.allow_headers = value.split(',');
    }
  }
});
</script>

<template>
  <div class="config-section">
    <h2>跨域配置</h2>
    <n-form>
      <n-form-item label="允许来源:" path="allowOrigins">
        <n-input
          v-model:value="allowOrigins"
          placeholder="逗号分隔的来源"
        />
      </n-form-item>
      <n-form-item label="允许方法:" path="allowMethods">
        <n-input
          v-model:value="allowMethods"
          placeholder="逗号分隔的方法"
        />
      </n-form-item>
      <n-form-item label="允许头:" path="allowHeaders">
        <n-input
          v-model:value="allowHeaders"
          placeholder="逗号分隔的头"
        />
      </n-form-item>
    </n-form>
  </div>
</template>

<script setup lang="ts">
import { defineProps, computed } from 'vue';
import { NInput, NForm, NFormItem } from 'naive-ui';

const props = defineProps<{
  config: {
    cors: {
      allow_origins: string[];
      allow_methods: string[];
      allow_headers: string[];
    };
  };
}>();

const allowOrigins = computed({
  get() {
    return props.config.cors.allow_origins.join(',');
  },
  set(value) {
    props.config.cors.allow_origins = value.split(',');
  }
});

const allowMethods = computed({
  get() {
    return props.config.cors.allow_methods.join(',');
  },
  set(value) {
    props.config.cors.allow_methods = value.split(',');
  }
});

const allowHeaders = computed({
  get() {
    return props.config.cors.allow_headers.join(',');
  },
  set(value) {
    props.config.cors.allow_headers = value.split(',');
  }
});
</script>

<template>
  <div class="config-section">
    <h2>CORS Configuration</h2>
    <n-form>
      <n-form-item label="Allow Origins:" path="allowOrigins">
        <n-input
          v-model:value="allowOrigins"
          placeholder="Comma separated origins"
        />
      </n-form-item>
      <n-form-item label="Allow Methods:" path="allowMethods">
        <n-input
          v-model:value="allowMethods"
          placeholder="Comma separated methods"
        />
      </n-form-item>
      <n-form-item label="Allow Headers:" path="allowHeaders">
        <n-input
          v-model:value="allowHeaders"
          placeholder="Comma separated headers"
        />
      </n-form-item>
    </n-form>
  </div>
</template>

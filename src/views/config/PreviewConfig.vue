<script setup lang="ts">
import { computed } from 'vue';
import { NCard, NSpace } from 'naive-ui';

interface Config {
  scheme?: {
    address: string;
    http_port: number;
    https_port: number;
    force_https: boolean;
    cert_file: string;
    key_file: string;
    unix_file: string;
    unix_file_perm: string;
  };
  // 其他配置接口可以根据实际配置添加
}

const props = defineProps<{
  config: Config;
}>();

const schemeConfig = computed(() => props.config?.scheme);
</script>

<template>
    <n-space vertical>
      <n-card v-if="schemeConfig" class="preview-section">
        <div class="preview-content">
          <template v-if="schemeConfig.unix_file">
            <p>Unix Socket: {{ schemeConfig.unix_file }}</p>
            <p v-if="schemeConfig.unix_file_perm">
              Permissions: {{ schemeConfig.unix_file_perm }}
            </p>
          </template>
          <template v-else>
            <p>
              Server URL: {{ schemeConfig.force_https ? 'https' : 'http' }}://{{
                schemeConfig.address || 'localhost'
              }}:{{ schemeConfig.force_https ? schemeConfig.https_port : schemeConfig.http_port }}
            </p>
            <p v-if="schemeConfig.cert_file && schemeConfig.key_file">
              SSL Certificate Configured
            </p>
          </template>
        </div>
      </n-card>

      <!-- 其他配置部分的预览可以在这里添加 -->
    </n-space>
  </n-card>
</template>

<style scoped>
.preview-container {
  margin-top: 24px;
}

.preview-section {
  margin-bottom: 16px;
}

.preview-content {
  line-height: 1.8;
  padding: 8px;
}
</style>

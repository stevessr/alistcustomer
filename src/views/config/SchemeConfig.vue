<script setup lang="ts">
import { computed, ref } from 'vue';

interface SchemeConfig {
  address: string;
  http_port: number;
  https_port: number;
  force_https: boolean;
  cert_file: string;
  key_file: string;
  unix_file: string;
  unix_file_perm: string;
}

interface Config {
  scheme?: SchemeConfig;
}

const { config = {
  scheme: {
    address: '',
    http_port: 80,
    https_port: 443,
    force_https: false,
    cert_file: '',
    key_file: '',
    unix_file: '',
    unix_file_perm: ''
  }
} } = defineProps<{
  config?: Config;
}>();

const address = computed({
  get: () => config?.scheme?.address ?? '',
  set: (value: string) => {
    if (config?.scheme) config.scheme.address = value;
  }
});

const httpPort = computed({
  get: () => config?.scheme?.http_port ?? 80,
  set: (value: number) => {
    if (config?.scheme) config.scheme.http_port = value;
  }
});

const httpsPort = computed({
  get: () => config?.scheme?.https_port ?? 443,
  set: (value: number) => {
    if (config?.scheme) config.scheme.https_port = value;
  }
});

const forceHttps = computed({
  get: () => config?.scheme?.force_https ?? false,
  set: (value: boolean) => {
    if (config?.scheme) config.scheme.force_https = value;
  }
});

const certFile = computed({
  get: () => config?.scheme?.cert_file ?? '',
  set: (value: string) => {
    if (config?.scheme) config.scheme.cert_file = value;
  }
});

const keyFile = computed({
  get: () => config?.scheme?.key_file ?? '',
  set: (value: string) => {
    if (config?.scheme) config.scheme.key_file = value;
  }
});

const unixFile = computed({
  get: () => config?.scheme?.unix_file ?? '',
  set: (value: string) => {
    if (config?.scheme) config.scheme.unix_file = value;
  }
});

const unixFilePerm = computed({
  get: () => config?.scheme?.unix_file_perm ?? '',
  set: (value: string) => {
    if (config?.scheme) config.scheme.unix_file_perm = value;
  }
});

const isPreview = ref(false);
</script>

<template>
  <div class="config-section">
    <div class="config-header">
      <h2>Scheme Configuration</h2>
      <n-switch
        v-model:checked="isPreview"
        :round="false"
        size="small"
      >
        <template #checked>Preview</template>
        <template #unchecked>Edit</template>
      </n-switch>
    </div>
    <n-form>
      <n-form-item label="Address:" path="schemeAddress">
  <n-input
    v-model:value="address"
    placeholder="Enter server address"
    :disabled="isPreview"
  />
      </n-form-item>

      <n-form-item label="HTTP Port:" path="schemeHttpPort">
        <n-input-number
          v-model:value="httpPort"
          :min="0"
          :max="65535"
          :disabled="isPreview"
        />
      </n-form-item>

      <n-form-item label="HTTPS Port:" path="schemeHttpsPort">
        <n-input-number
          v-model:value="httpsPort"
          :min="0"
          :max="65535"
          :disabled="isPreview"
        />
      </n-form-item>

      <n-form-item label="Force HTTPS:" path="schemeForceHttps">
        <n-switch v-model:checked="forceHttps" :disabled="isPreview" />
      </n-form-item>

      <n-form-item label="Cert File:" path="schemeCertFile">
        <n-input
          v-model:value="certFile"
          placeholder="Enter certificate file path"
          :disabled="isPreview"
        />
      </n-form-item>

      <n-form-item label="Key File:" path="schemeKeyFile">
        <n-input
          v-model:value="keyFile"
          placeholder="Enter key file path"
          :disabled="isPreview"
        />
      </n-form-item>

      <n-form-item label="Unix File:" path="schemeUnixFile">
        <n-input
          v-model:value="unixFile"
          placeholder="Enter Unix socket file path"
          :disabled="isPreview"
        />
      </n-form-item>

      <n-form-item label="Unix File Perm:" path="schemeUnixFilePerm">
        <n-input
          v-model:value="unixFilePerm"
          placeholder="Enter Unix file permissions"
          :disabled="isPreview"
        />
      </n-form-item>
    </n-form>

    <PreviewConfig v-if="isPreview" :config="config" />
  </div>
</template>

<style scoped>
.config-section {
  margin-bottom: 24px;
}

.config-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 16px;
}
</style>

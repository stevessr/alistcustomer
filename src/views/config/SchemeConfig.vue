<script setup lang="ts">
import { defineProps } from 'vue';

defineProps<{
  config: {
    scheme: {
      address: string;
      http_port: number;
      https_port: number;
      force_https: boolean;
      cert_file: string;
      key_file: string;
      unix_file: string;
      unix_file_perm: string;
    };
  };
}>();
</script>

<template>
  <div class="config-section">
    <h2>Scheme Configuration</h2>
    <n-form>
      <n-form-item label="Address:" path="schemeAddress">
        <n-input
          v-model:value="config.scheme.address"
          placeholder="Enter server address"
        />
      </n-form-item>

      <n-form-item label="HTTP Port:" path="schemeHttpPort">
        <n-input-number
          v-model:value="config.scheme.http_port"
          :min="0"
          :max="65535"
        />
      </n-form-item>

      <n-form-item label="HTTPS Port:" path="schemeHttpsPort">
        <n-input-number
          v-model:value="config.scheme.https_port"
          :min="0"
          :max="65535"
        />
      </n-form-item>

      <n-form-item label="Force HTTPS:" path="schemeForceHttps">
        <n-checkbox v-model:checked="config.scheme.force_https" />
      </n-form-item>

      <n-form-item label="Cert File:" path="schemeCertFile">
        <n-input
          v-model:value="config.scheme.cert_file"
          placeholder="Enter certificate file path"
        />
      </n-form-item>

      <n-form-item label="Key File:" path="schemeKeyFile">
        <n-input
          v-model:value="config.scheme.key_file"
          placeholder="Enter key file path"
        />
      </n-form-item>

      <n-form-item label="Unix File:" path="schemeUnixFile">
        <n-input
          v-model:value="config.scheme.unix_file"
          placeholder="Enter Unix socket file path"
        />
      </n-form-item>

      <n-form-item label="Unix File Perm:" path="schemeUnixFilePerm">
        <n-input
          v-model:value="config.scheme.unix_file_perm"
          placeholder="Enter Unix file permissions"
        />
      </n-form-item>
    </n-form>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import { 
  NForm, 
  NFormItem, 
  NInput, 
  NSwitch, 
  NButton,
  useMessage
} from 'naive-ui';

interface SftpConfig {
  enable: boolean;
  listen: string;
}

interface Config {
  sftp: SftpConfig;
}

const message = useMessage();
const loading = ref(false);
const props = defineProps<{
  config: Config;
}>();

const handleReset = async () => {
  try {
    loading.value = true;
    // Simulate async reset operation
    await new Promise((resolve) => setTimeout(resolve, 1000));
    
    // Reset values
    props.config.sftp.enable = false;
    props.config.sftp.listen = '127.0.0.1:22';
    
    message.success('SFTP settings reset to defaults');
  } catch (error) {
    message.error('Failed to reset SFTP settings');
    console.error('Reset error:', error);
  } finally {
    loading.value = false;
  }
};
</script>

<template>
  <div class="config-section">
    <h2>SFTP Configuration</h2>
    
    <n-form
      label-placement="left"
      label-width="auto"
      :show-feedback="true"
    >
      <n-form-item 
        label="Enable SFTP" 
        path="sftpEnable"
        :feedback="'Enable or disable SFTP file transfer service'"
      >
        <n-switch
          v-model:value="config.sftp.enable"
          size="medium"
          :loading="loading"
        />
      </n-form-item>

      <n-form-item 
        label="Listen Address" 
        path="sftpListen"
        :feedback="'Format: IP:Port (e.g. 127.0.0.1:22)'"
        :rule="{
          pattern: /^([0-9]{1,3}\.){3}[0-9]{1,3}:[0-9]{1,5}$/,
          message: 'Must be in format IP:Port (e.g. 127.0.0.1:22)',
          required: true,
          trigger: ['input', 'blur']
        }"
      >
        <n-input
          v-model:value="config.sftp.listen"
          placeholder="127.0.0.1:22"
          clearable
          :loading="loading"
        />
      </n-form-item>

      <n-form-item>
        <n-button
          type="primary"
          :loading="loading"
          @click="handleReset"
        >
          Reset to Defaults
        </n-button>
      </n-form-item>
    </n-form>
  </div>
</template>

<style scoped>
.n-form-item {
  max-width: 600px;
}

.n-button {
  margin-top: 16px;
}
</style>

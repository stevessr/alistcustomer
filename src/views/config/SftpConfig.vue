<script setup lang="ts">
import { ref, computed } from 'vue';
import { 
  NForm, 
  NFormItem, 
  NInput, 
  NSwitch, 
  NButton,
  useMessage
} from 'naive-ui';

interface SftpConfig {
  enable?: boolean;
  listen?: string;
}

interface Config {
  sftp?: SftpConfig;
}

const message = useMessage();
const loading = ref(false);
const props = withDefaults(defineProps<{
  config?: Config;
}>(), {
  config: () => ({
    sftp: {
      enable: false,
      listen: '127.0.0.1:22'
    }
  })
});

const sftpEnabled = computed({
  get: () => props.config?.sftp?.enable ?? false,
  set: (val: boolean) => {
    if (props.config?.sftp) {
      props.config.sftp.enable = val;
    }
  }
});

const sftpListenAddress = computed({
  get: () => props.config?.sftp?.listen ?? '127.0.0.1:22',
  set: (val: string) => {
    if (props.config?.sftp) {
      props.config.sftp.listen = val;
    }
  }
});

const handleReset = async () => {
  try {
    loading.value = true;
    // Simulate async reset operation
    await new Promise((resolve) => setTimeout(resolve, 1000));
    
    // Reset values
    props.config!.sftp!.enable = false;
    props.config!.sftp!.listen = '127.0.0.1:22';
    
    message.success('SFTP设置已重置为默认值');
  } catch (error) {
    message.error('重置SFTP设置失败');
    console.error('Reset error:', error);
  } finally {
    loading.value = false;
  }
};
</script>

<template>
  <div class="config-section">
    <h2>SFTP配置</h2>
    
    <n-form
      label-placement="left"
      label-width="auto"
      :show-feedback="true"
    >
      <n-form-item 
        label="启用SFTP" 
        path="sftpEnable"
        :feedback="'启用或禁用SFTP文件传输服务'"
      >
        <n-switch
          v-model:value="sftpEnabled"
          size="medium"
          :loading="loading"
        />
      </n-form-item>

      <n-form-item 
        label="监听地址" 
        path="sftpListen"
        :feedback="'格式: IP:端口 (例如 127.0.0.1:22)'"
        :rule="{
          pattern: /^([0-9]{1,3}\.){3}[0-9]{1,3}:[0-9]{1,5}$/,
          message: '必须为IP:端口格式 (例如 127.0.0.1:22)',
          required: true,
          trigger: ['input', 'blur']
        }"
      >
        <n-input
          v-model:value="sftpListenAddress"
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
          重置为默认值
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

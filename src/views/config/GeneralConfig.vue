<script setup lang="ts">
import { ref } from 'vue';
import {
  NForm,
  NFormItem,
  NInput,
  NSwitch,
  NButton,
  useNotification
} from 'naive-ui';

interface Config {
  force: boolean;
  site_url: string;
  cdn: string;
}

const notification = useNotification();
const loading = ref(false);
const saving = ref(false);
const props = withDefaults(defineProps<{
  config: Config;
}>(), {
  config: () => ({
    force: false,
    site_url: '',
    cdn: ''
  })
});

const handleReset = async () => {
  try {
    loading.value = true;
    await new Promise((resolve) => setTimeout(resolve, 1000));
    
    props.config.force = false;
    props.config.site_url = '';
    props.config.cdn = '';
    
    notification.success({
      content: '重置成功',
      meta: '所有常规设置已重置为默认值',
      duration: 3000
    });
  } catch (error) {
    notification.error({
      content: '重置失败',
      meta: '重置设置时发生错误',
      duration: 5000
    });
    console.error('Reset error:', error);
  } finally {
    loading.value = false;
  }
};

const handleSave = async () => {
  try {
    saving.value = true;
    await new Promise((resolve) => setTimeout(resolve, 1000));
    
    notification.success({
      content: '保存成功',
      meta: '常规设置已保存',
      duration: 3000
    });
  } catch (error) {
    notification.error({
      content: '保存失败',
      meta: '保存设置时发生错误',
      duration: 5000
    });
    console.error('Save error:', error);
  } finally {
    saving.value = false;
  }
};
</script>

<template>
  <div class="config-section">
    <h2>常规配置</h2>
    
    <n-form
      label-placement="left"
      label-width="auto"
      :show-feedback="true"
    >
      <n-form-item 
        label="强制模式" 
        path="force"
        :feedback="'启用以强制某些操作'"
      >
        <n-switch v-model:value="config.force" />
      </n-form-item>

      <n-form-item 
        label="站点URL" 
        path="site_url"
        :rule="{
          type: 'url',
          message: '请输入以http://或https://开头的有效URL',
          trigger: ['input', 'blur'],
          required: true,
          validator: (_rule: unknown, value: string) => {
            if (!value) return false;
            const pattern = /^https?:\/\/(www\.)?[-a-zA-Z0-9@:%._\+~#=]{1,256}\.[a-zA-Z0-9()]{1,6}\b([-a-zA-Z0-9()@:%_\+.~#?&//=]*)$/;
            return pattern.test(value);
          }
        }"
      >
        <n-input
          v-model:value="config.site_url"
          placeholder="https://example.com"
          clearable
          :loading="loading"
          :status="config.site_url ? 'success' : 'warning'"
        >
          <template #prefix>
            <n-icon>
              <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path fill="currentColor" d="M12 2a10 10 0 0 1 10 10a10 10 0 0 1-10 10A10 10 0 0 1 2 12A10 10 0 0 1 12 2m0 2a8 8 0 0 0-8 8c0 2.21 1.79 4 4 4c1.18 0 2.25-.5 3-1.32c.75.82 1.82 1.32 3 1.32c2.21 0 4-1.79 4-4c0-4.42-3.58-8-8-8m0 2a6 6 0 0 1 6 6c0 1.1-.9 2-2 2c-.64 0-1.23-.3-1.61-.8c-.38.5-.97.8-1.61.8c-.64 0-1.23-.3-1.61-.8c-.38.5-.97.8-1.61.8c-1.1 0-2-.9-2-2a6 6 0 0 1 6-6m0 2a4 4 0 0 0-4 4c0 .55.45 1 1 1c.28 0 .53-.11.71-.29c.18.18.43.29.71.29c.28 0 .53-.11.71-.29c.18.18.43.29.71.29c.28 0 .53-.11.71-.29c.18.18.43.29.71.29c.55 0 1-.45 1-1a4 4 0 0 0-4-4m0 2a2 2 0 0 1 2 2c0 .28-.22.5-.5.5c-.14 0-.26-.06-.35-.15c-.09.09-.21.15-.35.15c-.14 0-.26-.06-.35-.15c-.09.09-.21.15-.35.15c-.14 0-.26-.06-.35-.15c-.09.09-.21.15-.35.15c-.28 0-.5-.22-.5-.5a2 2 0 0 1 2-2Z"/></svg>
            </n-icon>
          </template>
        </n-input>
      </n-form-item>

      <n-form-item 
        label="CDN URL" 
        path="cdn"
        :rule="{
          type: 'url',
          message: '请输入以http://或https://开头的有效URL',
          trigger: ['input', 'blur']
        }"
      >
        <n-input
          v-model:value="config.cdn"
          placeholder="https://cdn.example.com"
          clearable
          :loading="loading"
        />
      </n-form-item>

      <n-form-item>
        <n-space>
          <n-button
            type="primary"
            :loading="saving"
            @click="handleSave"
          >
            保存更改
          </n-button>
          <n-button
            secondary
            type="warning"
            :loading="loading"
            @click="handleReset"
          >
            重置为默认值
          </n-button>
        </n-space>
      </n-form-item>
    </n-form>
  </div>
</template>

<style scoped>
.config-section {
  max-width: 800px;
  margin: 0 auto;
}

.n-form {
  display: grid;
  gap: 24px;
  padding: 16px;
}

.n-form-item {
  max-width: 100%;
}

.n-space {
  gap: 12px;
  margin-top: 16px;
}
</style>

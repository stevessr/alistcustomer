<script setup lang="ts">
import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import ConfigColorPicker from './ConfigColorPicker.vue';
import ConfigDraggableSections from './ConfigDraggableSections.vue';
import ConfigActions from './ConfigActions.vue';
import GeneralConfig from './GeneralConfig.vue';
import DatabaseConfig from './DatabaseConfig.vue';
import MeilisearchConfig from './MeilisearchConfig.vue';
import SchemeConfig from './SchemeConfig.vue';
import LogConfig from './LogConfig.vue';
import TaskConfig from './TaskConfig.vue';
import CorsConfig from './CorsConfig.vue';
import S3Config from './S3Config.vue';
import FtpConfig from './FtpConfig.vue';
import SftpConfig from './SftpConfig.vue';
import { defaultColors } from './constants';

interface SectionColors {
  general: string;
  database: string;
  meilisearch: string;
  scheme: string;
  log: string;
  task: string;
  cors: string;
  s3: string;
  ftp: string;
  sftp: string;
}

interface Config {
  section_colors: SectionColors;
  general?: {
    force: boolean;
    site_url: string;
    cdn: string;
  };
  s3: {
    enable: boolean;
    port: number;
    ssl: boolean;
  };
  sftp: {
    enable: boolean;
    listen: string;
  };
  ftp: {
    enable: boolean;
    listen: string;
    find_pasv_port_attempts: number;
    active_transfer_port_non_20: boolean;
    idle_timeout: number;
    connection_timeout: number;
    disable_active_mode: boolean;
    default_transfer_binary: boolean;
    enable_active_conn_ip_check: boolean;
    enable_pasv_conn_ip_check: boolean;
  };
}

const config = ref<Config>({
  section_colors: defaultColors,
  s3: {
    enable: false,
    port: 9000,
    ssl: false
  },
  sftp: {
    enable: false,
    listen: '0.0.0.0:22'
  },
  ftp: {
    enable: false,
    listen: '0.0.0.0:21',
    find_pasv_port_attempts: 5,
    active_transfer_port_non_20: false,
    idle_timeout: 300,
    connection_timeout: 60,
    disable_active_mode: false,
    default_transfer_binary: true,
    enable_active_conn_ip_check: false,
    enable_pasv_conn_ip_check: false
  }
});

// 初始化配置
function deepMerge(target: any, source: any) {
  for (const key of Object.keys(source)) {
    if (source[key] instanceof Object && key in target) {
      Object.assign(source[key], deepMerge(target[key], source[key]));
    }
  }
  Object.assign(target || {}, source);
  return target;
}

invoke('read_config').then((data: any) => {
  if (data) {
    config.value = deepMerge(config.value, data);
  }
}).catch((error) => {
  console.error('Failed to load config:', error);
});

// 只保留一个 configComponents 定义
const configComponents = ref([
  { id: 'general', component: GeneralConfig, props: { config: config.value?.general } },
  { id: 'database', component: DatabaseConfig, props: { config } },
  { id: 'meilisearch', component: MeilisearchConfig, props: { config } },
  { id: 'scheme', component: SchemeConfig, props: { config } },
  { id: 'log', component: LogConfig, props: { config } },
  { id: 'task', component: TaskConfig, props: { config } },
  { id: 'cors', component: CorsConfig, props: { config } },
  { id: 's3', component: S3Config, props: { config } },
  { id: 'ftp', component: FtpConfig, props: { config } },
  { id: 'sftp', component: SftpConfig, props: { config } }
]);

// 保持一个 saveColors 定义
const saveColors = () => {
  localStorage.setItem('section_colors', JSON.stringify(config.value?.section_colors));
};

// 保持一个 handleSave 定义
const handleSave = async () => {
  try {
    saveColors();
    if (config.value) {
      await invoke('write_config', { config: config.value });
      alert('Config saved successfully!');
    }
  } catch (error) {
    alert(`Failed to save config: ${error}`);
  }
};

// 初始化配置和加载配置的逻辑...
</script>

<template>
  <div class="config-container">
    <h1 class="config-title">Config Editor</h1>
    <ConfigColorPicker 
      :colors="config?.section_colors || {}"
      @update:colors="(newColors) => { if (config) config.section_colors = newColors }"
    />
<ConfigDraggableSections 
  :components="configComponents"
  :colors="config?.section_colors || {}"
  :config="config"
/>
    <ConfigActions @save-config="handleSave" />
  </div>
</template>

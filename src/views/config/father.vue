<script setup lang="ts">
import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import draggable from 'vuedraggable';
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

const configComponents = ref([
  { id: 'general', component: GeneralConfig },
  { id: 'database', component: DatabaseConfig },
  { id: 'meilisearch', component: MeilisearchConfig },
  { id: 'scheme', component: SchemeConfig },
  { id: 'log', component: LogConfig },
  { id: 'task', component: TaskConfig },
  { id: 'cors', component: CorsConfig },
  { id: 's3', component: S3Config },
  { id: 'ftp', component: FtpConfig },
  { id: 'sftp', component: SftpConfig }
]);

interface Config {
  force: boolean;
  site_url: string;
  cdn: string;
  jwt_secret: string;
  token_expires_in: number;
  section_colors: {
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
  };
  database: {
    type: 'mysql' | 'postgres' | 'sqlite';
    host: string;
    port?: number;
    user: string;
    password: string;
    name: string;
    dbFile: string;
    tablePrefix: string;
    sslMode: 'disable' | 'require' | 'verify-full';
    dsn: string;
  };
  meilisearch: {
    host: string;
    api_key: string;
    index_prefix: string;
  };
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
  temp_dir: string;
  bleve_dir: string;
  dist_dir: string;
  log: {
    enable: boolean;
    name: string;
    max_size: number;
    max_backups: number;
    max_age: number;
    compress: boolean;
  };
  delayed_start: number;
  max_connections: number;
  tls_insecure_skip_verify: boolean;
  tasks: {
    download: {
      workers: number;
      max_retry: number;
      task_persistant: boolean;
    };
    transfer: {
      workers: number;
      max_retry: number;
      task_persistant: boolean;
    };
    upload: {
      workers: number;
      max_retry: number;
      task_persistant: boolean;
    };
    copy: {
      workers: number;
      max_retry: number;
      task_persistant: boolean;
    };
  };
  cors: {
    allow_origins: string[];
    allow_methods: string[];
    allow_headers: string[];
  };
  s3: {
    enable: boolean;
    port: number;
    ssl: boolean;
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
  sftp: {
    enable: boolean;
    listen: string;
  };
}

const defaultColors = {
  general: '#18a058',
  database: '#2080f0', 
  meilisearch: '#18a058',
  scheme: '#f0a020',
  log: '#d03050',
  task: '#9c27b0',
  cors: '#00bcd4',
  s3: '#ff9800',
  ftp: '#4caf50',
  sftp: '#3f51b5'
};

const config = ref<Config>({
  force: false,
  site_url: '',
  cdn: '',
  jwt_secret: '',
  token_expires_in: 0,
  section_colors: JSON.parse(localStorage.getItem('section_colors') || JSON.stringify(defaultColors)),
  database: {
    type: 'mysql',
    host: '',
    port: 0,
    user: '',
    password: '',
    name: '',
    dbFile: '',
    tablePrefix: '',
    sslMode: 'disable',
    dsn: ''
  },
  meilisearch: {
    host: '',
    api_key: '',
    index_prefix: ''
  },
  scheme: {
    address: '',
    http_port: 0,
    https_port: 0,
    force_https: false,
    cert_file: '',
    key_file: '',
    unix_file: '',
    unix_file_perm: ''
  },
  temp_dir: '',
  bleve_dir: '',
  dist_dir: '',
  log: {
    enable: false,
    name: '',
    max_size: 0,
    max_backups: 0,
    max_age: 0,
    compress: false
  },
  delayed_start: 0,
  max_connections: 0,
  tls_insecure_skip_verify: false,
  tasks: {
    download: {
      workers: 0,
      max_retry: 0,
      task_persistant: false
    },
    transfer: {
      workers: 0,
      max_retry: 0,
      task_persistant: false
    },
    upload: {
      workers: 0,
      max_retry: 0,
      task_persistant: false
    },
    copy: {
      workers: 0,
      max_retry: 0,
      task_persistant: false
    }
  },
  cors: {
    allow_origins: [],
    allow_methods: [],
    allow_headers: []
  },
  s3: {
    enable: false,
    port: 0,
    ssl: false
  },
  ftp: {
    enable: false,
    listen: '',
    find_pasv_port_attempts: 0,
    active_transfer_port_non_20: false,
    idle_timeout: 0,
    connection_timeout: 0,
    disable_active_mode: false,
    default_transfer_binary: false,
    enable_active_conn_ip_check: false,
    enable_pasv_conn_ip_check: false
  },
  sftp: {
    enable: false,
    listen: ''
  }
});

const loadConfig = async () => {
  try {
    config.value = await invoke<Config>('read_config');
  } catch (error) {
    console.error('Failed to load config:', error);
  }
};

const saveColors = () => {
  localStorage.setItem('section_colors', JSON.stringify(config.value.section_colors));
};

const saveConfig = async () => {
  try {
    saveColors();
    await invoke('write_config', { config: config.value });
    alert('Config saved successfully!');
  } catch (error) {
    alert(`Failed to save config: ${error}`);
  }
};

loadConfig();
</script>

<template>
  <div class="config-container">
    <h1 class="config-title">Config Editor</h1>
    <div class="color-picker-container">
      <div v-for="(color, section) in config.section_colors" :key="section" class="color-picker">
        <label :for="`${section}-color`">{{ section }}:</label>
        <input
          type="color"
          :id="`${section}-color`"
          v-model="config.section_colors[section]"
          @change="saveColors"
        />
      </div>
    </div>
    <div class="config-layout">
    <div class="config-grid">
      <draggable 
        v-model="configComponents"
        item-key="id"
        handle=".drag-handle"
        class="draggable-container"
      >
        <template #item="{ element }">
          <div class="config-section-wrapper">
            <div class="drag-handle">☰</div>
            <component 
              :is="element.component" 
              v-model:config="config" 
              class="config-section"
              :data-type="element.id"
            />
          </div>
        </template>
      </draggable>
      
      <div class="config-actions-side">
        <n-button 
          type="submit" 
          class="btn btn-primary"
          @click="saveConfig"
        >
          Save Configuration
        </n-button>
      </div>
    </div>
    </div>
  </div>
</template>

<style scoped>
.draggable-container {
  display: flex;
  flex-direction: column;
  gap: 24px;
}

.config-section-wrapper {
  position: relative;
  background: var(--n-color-modal);
  border-radius: var(--n-border-radius);
  box-shadow: var(--n-box-shadow);
}

.drag-handle {
  position: absolute;
  left: 8px;
  top: 8px;
  cursor: move;
  padding: 8px;
  color: var(--n-text-color);
  opacity: 0.5;
  transition: opacity 0.2s ease;
}

.drag-handle:hover {
  opacity: 1;
}

.color-picker-container {
  display: flex;
  flex-wrap: wrap;
  gap: 12px;
  margin-bottom: 24px;
}

.color-picker {
  display: flex;
  align-items: center;
  gap: 8px;
}

.color-picker label {
  font-weight: 500;
  color: var(--n-text-color);
}

.config-section {
  padding: 24px;
  padding-left: 40px;
  border-left: 4px solid;
}

.config-section[data-type="general"] {
  border-color: v-bind('config.section_colors.general');
}

.config-section[data-type="database"] {
  border-color: v-bind('config.section_colors.database');
}

.config-section[data-type="meilisearch"] {
  border-color: v-bind('config.section_colors.meilisearch');
}

.config-section[data-type="scheme"] {
  border-color: v-bind('config.section_colors.scheme');
}

.config-section[data-type="log"] {
  border-color: v-bind('config.section_colors.log');
}

.config-section[data-type="task"] {
  border-color: v-bind('config.section_colors.task');
}

.config-section[data-type="cors"] {
  border-color: v-bind('config.section_colors.cors');
}

.config-section[data-type="s3"] {
  border-color: v-bind('config.section_colors.s3');
}

.config-section[data-type="ftp"] {
  border-color: v-bind('config.section_colors.ftp');
}

.config-section[data-type="sftp"] {
  border-color: v-bind('config.section_colors.sftp');
}
</style>

<style scoped>
@import './config.css';
</style>

<script setup lang="ts">
import type { Component } from "vue";
import { shallowRef, markRaw, ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { NTabs, NTabPane } from "naive-ui";
import ConfigColorPicker from "./ConfigColorPicker.vue";
import ConfigActions from "./ConfigActions.vue";
import ConfigPreviewDialog from "./ConfigPreviewDialog.vue";
import GeneralConfig from "./GeneralConfig.vue";
import DatabaseConfig from "./DatabaseConfig.vue";
import MeilisearchConfig from "./MeilisearchConfig.vue";
import SchemeConfig from "./SchemeConfig.vue";
import LogConfig from "./LogConfig.vue";
import TaskConfig from "./TaskConfig.vue";
import CorsConfig from "./CorsConfig.vue";
import S3Config from "./S3Config.vue";
import FtpConfig from "./FtpConfig.vue";
import SftpConfig from "./SftpConfig.vue";
import { defaultColors } from "./constants";
import ThemeToggle from "../../components/ThemeToggle.vue";

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
  meilisearch?: {
    enable: boolean;
    host: string;
    api_key: string;
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
  database: {
    type: string;
    host: string; // 数据库服务器地址
    port: number; // 数据库端口
    user: string; // 认证用户名
    password?: string; // 认证密码
    name: string;
    db_file: string;
    table_prefix: string;
    ssl_mode: string; // SSL 模式（根据数据库类型可选值不同）
    dsn: string; // 自定义连接字符串
  };
  [key: string]: any; // Add index signature
}

const config = ref<Config>({
  section_colors: defaultColors,
  s3: {
    enable: false,
    port: 9000,
    ssl: false,
  },
  sftp: {
    enable: false,
    listen: "0.0.0.0:22",
  },
  ftp: {
    enable: false,
    listen: "0.0.0.0:21",
    find_pasv_port_attempts: 5,
    active_transfer_port_non_20: false,
    idle_timeout: 300,
    connection_timeout: 60,
    disable_active_mode: false,
    default_transfer_binary: true,
    enable_active_conn_ip_check: false,
    enable_pasv_conn_ip_check: false,
  },
  database: {  // 保证初始数据结构
    type: "sqlite3",
    host: "",
    port: 0,
    user: "",
    password: "",
    name: "",
    db_file: "data\\data.db",
    table_prefix: "x_",
    ssl_mode: "disable",
    dsn: ""
  }
});

const previewDialog = ref<
  InstanceType<typeof ConfigPreviewDialog> & {
    showPreview: (config: Config) => void;
  }
>();

// 初始化配置
function deepMerge(target: any, source: any) {
  for (const key of Object.keys(source)) {
    if (source[key] instanceof Object) {
      if (!target[key]) target[key] = {};
      deepMerge(target[key], source[key]);
    } else {
      target[key] = source[key];
    }
    if (import.meta.env.MODE === "development") {
      if (key) {
        // 添加类型检查
        console.log("valid key type:", key, "in", source);
        continue;
      }
    }
  }
  return target;
}

invoke("read_config")
  .then((data: any) => {
    if (data?.database) {
      data.database.port = Number(data.database.port) || 0;
    }
    if (data) {
      const merged = deepMerge({ ...config.value }, data);
      config.value = merged;
      if (import.meta.env.MODE === "development") {
        console.log("Loaded config:", config.value);
      }
    }
  })
  .catch((error) => {
    console.error("Failed to load config:", error);
    if (import.meta.env.MODE === "development") {
      console.error("Error details:", error);
    }
  });

// 只保留一个 configComponents 定义
// 添加更严格的类型定义
interface ConfigComponent {
  id: keyof SectionColors;
  component: Component;
  props: () => Record<string, unknown>;
}

const configComponents = shallowRef<ConfigComponent[]>([
  {
    id: "general",
    component: markRaw(GeneralConfig),
    props: () => ({ config: config.value.general }),
  },
  {
    id: "database",
    component: markRaw(DatabaseConfig),
    props: () => ({
      config: config.value?.database ?? {
        // 添加双重保护
        type: "sqlite3",
        host: "",
        port: 0,
        user: "",
        password: "",
        name: "",
        db_file: "data\\data.db",
        table_prefix: "x_",
        ssl_mode: "",
        dsn: "",
      },
    }),
  },
  {
    id: "meilisearch",
    component: markRaw(MeilisearchConfig),
    props: () => ({ config: config.value.meilisearch }),
  },
  // 其他配置项保持相同模式，统一使用函数返回最新配置
  ...[
    SchemeConfig,
    LogConfig,
    TaskConfig,
    CorsConfig,
    S3Config,
    FtpConfig,
    SftpConfig,
  ].map((component, index) => ({
    id: ["scheme", "log", "task", "cors", "s3", "ftp", "sftp"][
      index
    ] as keyof SectionColors,
    component: markRaw(component),
    props: () => ({
      config:
        config.value[
          ["scheme", "log", "task", "cors", "s3", "ftp", "sftp"][index]
        ],
    }),
  })),
]);

// 保持一个 saveColors 定义
const saveColors = () => {
  if (config.value?.section_colors) {
    localStorage.setItem(
      "section_colors",
      JSON.stringify(config.value.section_colors)
    );
  }
};

// 保持一个 handleSave 定义
const handleSave = async () => {
  try {
    saveColors();
    if (config.value) {
      await invoke("write_config", { config: config.value });
      alert("Config saved successfully!");
    }
  } catch (error) {
    alert(`Failed to save config: ${error}`);
  }
};
</script>

<template>
  <div class="config-container">
    <div class="config-header">
      <ThemeToggle />
    </div>
    <ConfigColorPicker
      :colors="config?.section_colors || {}"
      @update:colors="
        (newColors) => {
          if (config) config.section_colors = newColors;
        }
      "
    />
    <n-tabs type="line" animated>
      <n-tab-pane
        v-for="tab in configComponents"
        :key="tab.id"
        :name="tab.id"
        :tab="tab.id.charAt(0).toUpperCase() + tab.id.slice(1)"
      >
        <component :is="tab.component" v-bind="tab.props()" />
      </n-tab-pane>
    </n-tabs>
    <ConfigActions
      @save-config="handleSave"
      @preview-config="previewDialog?.showPreview(config.value)"
    />
    <ConfigPreviewDialog ref="previewDialog" :config="config" />
  </div>
</template>

<style scoped>
.config-header {
  display: flex;
  align-items: center;
  gap: 12px;
  margin-bottom: 24px;
}

.config-container {
  max-width: 1200px;
  margin: 0 auto;
  padding: 24px;
}

@media (max-width: 768px) {
  .config-container {
    padding: 16px;
  }
}
</style>

<template>
  <div class="config-section">
    <h2>数据库配置</h2>
    <n-form>
      <n-form-item label="数据库类型:" path="dbType">
        <n-select
          v-if="config?.database"
          v-model:value="config.database.type"
          :options="[
            { label: 'MySQL', value: 'mysql' },
            { label: 'PostgreSQL', value: 'postgres' },
            { label: 'SQLite', value: 'sqlite3' },
          ]"
        />
        <n-alert v-else type="error">
          配置加载失败，请刷新页面或联系管理员
        </n-alert>
      </n-form-item>

      <template v-if="config?.database && config.database.type !== 'sqlite3'">
        <n-form-item label="主机:" path="dbHost">
          <n-input
            v-model:value="config.database.host"
            placeholder="localhost"
          />
        </n-form-item>
        <n-form-item label="端口:" path="dbPort">
          <n-input
            v-model:value="config.database.port"
            @update:value="(val: string) => {
            // Only allow numeric input and limit length
            const numericValue = val.replace(/\D/g, '').slice(0, 5);
            if (numericValue.length > 0) {
              // Validate port range using string length and first digit
              if (numericValue.length <= 5 && 
                  (numericValue.length < 5 || numericValue[0] <= '6')) {
                config.database.port = numericValue;
                return numericValue;
              }
            }
            config.database.port = '';
            return '';
          }"
            :placeholder="defaultPort"
            type="text"
          />
        </n-form-item>
        <n-form-item label="用户名:" path="dbUser">
          <n-input
            v-model:value="config.database.user"
            placeholder="数据库用户"
          />
        </n-form-item>
        <n-form-item label="密码:" path="dbPassword">
          <n-input
            v-model:value="config.database.password"
            type="password"
            placeholder="数据库密码"
          />
        </n-form-item>
      </template>

      <n-form-item label="数据库名称:" path="dbName">
        <n-input
          v-if="config?.database"
          v-model:value="config.database.name"
          placeholder="数据库名称"
        />
        <n-alert v-else type="error">
          配置加载失败，请刷新页面或联系管理员
        </n-alert>
      </n-form-item>

      <template v-if="config?.database && config.database.type === 'sqlite3'">
        <n-form-item label="数据库文件:" path="db_file">
          <n-input
            v-model:value="config.database.db_file"
            placeholder="数据库文件路径"
          />
        </n-form-item>
      </template>

      <n-form-item label="表前缀:" path="tablePrefix">
        <n-input
          v-if="config?.database"
          v-model:value="config.database.tablePrefix"
          placeholder="可选表前缀"
        />
        <n-alert v-else type="error">
          配置加载失败，请刷新页面或联系管理员
        </n-alert>
      </n-form-item>

      <template v-if="config?.database && config.database.type !== 'sqlite3'">
        <n-form-item label="SSL模式:" path="ssl_mode">
          <n-select
            v-model:value="config.database.ssl_mode"
            :options="[
              { label: '禁用', value: 'disable' },
              { label: '要求', value: 'require' },
              { label: '完全验证', value: 'verify-full' },
            ]"
          />
        </n-form-item>
        <n-form-item label="DSN:" path="dsn">
          <n-input
            v-model:value="config.database.dsn"
            placeholder="可选DSN字符串"
          />
        </n-form-item>
      </template>
    </n-form>
  </div>
</template>

<script setup lang="ts">
import { defineProps, computed } from "vue";
import { NForm, NFormItem, NSelect, NInput } from "naive-ui";

type DatabaseType = "mysql" | "postgres" | "sqlite3";
type SSLMode = "disable" | "require" | "verify-full";

interface DatabaseConfig {
  type: DatabaseType;
  host: string;
  port: string;
  user: string;
  password: string;
  name: string;
  db_file: string;
  tablePrefix: string;
  ssl_mode: SSLMode;
  dsn: string;
}

interface Config {
  database: DatabaseConfig;
}

const props = withDefaults(
  defineProps<{
    config: Config;
  }>(),
  {
    config: () => ({
      database: {
        type: "sqlite3",
        host: "",
        port: "0",
        user: "",
        password: "",
        name: "",
        db_file: "data\\data.db",
        tablePrefix: "x_",
        ssl_mode: "disable",
        dsn: "",
      },
    }),
  }
);

const defaultPort = computed(() => {
  switch (props.config.database.type) {
    case "mysql":
      return "3306";
    case "postgres":
      return "5432";
    default:
      return "";
  }
});
</script>

<style scoped>
.form-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
  gap: 1rem;
}
</style>

<template>
  <h>数据库配置</h>
  <div class="config-section">
    <n-form>
      <n-form-item label="数据库类型">
        <n-select
          :value="props.config.type.value"
          :options="[
            { label: 'MySQL', value: 'mysql' },
            { label: 'PostgreSQL', value: 'postgres' },
            { label: 'SQLite', value: 'sqlite3' },
          ]"
        />
      </n-form-item>

      <template v-if="config">
        <n-form-item label="主机:" path="dbHost">
          <n-input v-model:value="config.host" placeholder="localhost" />
        </n-form-item>
        <n-form-item label="端口:" path="dbPort">
          <n-input
            :value="String(config.port)"
            @update:value="handlePortInput"
            placeholder="请输入端口号"
            inputmode="numeric"
            pattern="[0-9]*"
          />
        </n-form-item>
        <n-form-item label="用户名:" path="dbUser">
          <n-input v-model:value="config.user" placeholder="数据库用户" />
        </n-form-item>
        <n-form-item label="密码:" path="dbPassword">
          <n-input
            v-model:value="config.password"
            type="password"
            placeholder="数据库密码"
          />
        </n-form-item>
      </template>

      <n-form-item label="数据库名称:" path="dbName">
        <n-input
          v-if="config"
          v-model:value="config.name"
          placeholder="数据库名称"
        />
        <n-alert v-else type="error">
          配置加载失败，请刷新页面或联系管理员
        </n-alert>
      </n-form-item>

      <template v-if="config && config.type === 'sqlite3'">
        <n-form-item label="数据库文件:" path="db_file">
          <n-input
            v-model:value="config.db_file"
            placeholder="数据库文件路径"
          />
        </n-form-item>
      </template>

      <n-form-item label="表前缀:" path="table_prefix">
        <n-input
          v-if="config"
          v-model:value="config.table_prefix"
          @update:value="(val) => config && (config.table_prefix = val)"
          placeholder="可选表前缀"
        />
      </n-form-item>

      <template v-if="config && config.type !== 'sqlite3'">
        <n-form-item label="SSL模式:" path="ssl_mode">
          <n-select
            v-model:value="config.ssl_mode"
            :options="[
              { label: '禁用', value: 'disable' },
              { label: '要求', value: 'require' },
              { label: '完全验证', value: 'verify-full' },
            ]"
          />
        </n-form-item>
        <n-form-item label="DSN:" path="dsn">
          <n-input v-model:value="config.dsn" placeholder="可选DSN字符串" />
        </n-form-item>
      </template>
    </n-form>
  </div>
</template>

<script setup lang="ts">
import { defineProps, computed, onMounted, toRef } from "vue";
import { NForm, NFormItem, NSelect, NInput } from "naive-ui";
import type { PropType } from "vue";

interface Config {
  database: {
    type: string;
    host: string; // 数据库服务器地址
    port: number; // 数据库端口
    user: string; // 认证用户名
    password: string; // 认证密码
    name: string;
    db_file: string;
    table_prefix: string;
    ssl_mode: string; // SSL 模式（根据数据库类型可选值不同）
    dsn: string; // 自定义连接字符串
  };
}

const props = defineProps({
  config: {
    type: Object as PropType<Config>,
    required: true,
    default: () => ({
      database: {
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
});

const config = toRef(props, "config");
const databaseConfig = computed(() => config.value);
const emit = defineEmits(["update:config"]);

const defaultPort = computed(() => {
  switch (props.config.type) {
    case "mysql":
      return "3306";
    case "postgres":
      return "5432";
    default:
      return "";
  }
});

const handlePortInput = (val: string | number) => {
  const numericValue = Math.min(Math.max(parseInt(String(val)) || 0, 0), 65535);
  // 使用 emit 更新配置
  emit("update:config", {
    ...config.value,
    database: {
      ...config.value.database,
      port: numericValue,
    },
  });
};

onMounted(() => {
  console.log("datbase:", props.config);
});
</script>

<style scoped>
.form-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
  gap: 1rem;
}
</style>

<template>
  <div class="config-section">
    <h2>Database Configuration</h2>
    <n-form>
      <n-form-item label="Database Type:" path="dbType">
        <n-select
          v-model:value="config.database.type"
          :options="[
            { label: 'MySQL', value: 'mysql' },
            { label: 'PostgreSQL', value: 'postgres' },
            { label: 'SQLite', value: 'sqlite' }
          ]"
        />
      </n-form-item>

      <template v-if="config.database.type !== 'sqlite'">
        <n-form-item label="Host:" path="dbHost">
          <n-input
            v-model:value="config.database.host"
            placeholder="localhost"
          />
        </n-form-item>
        <n-form-item label="Port:" path="dbPort">
        <n-input
          :value="config.database.port?.toString()"
          @update:value="val => config.database.port = parseInt(val)"
          :placeholder="defaultPort"
          :min="'0'"
          :max="'65535'"
        />
        </n-form-item>
        <n-form-item label="User:" path="dbUser">
          <n-input
            v-model:value="config.database.user"
            placeholder="Database user"
          />
        </n-form-item>
        <n-form-item label="Password:" path="dbPassword">
          <n-input
            v-model:value="config.database.password"
            type="password"
            placeholder="Database password"
          />
        </n-form-item>
      </template>

      <n-form-item label="Database Name:" path="dbName">
        <n-input
          v-model:value="config.database.name"
          placeholder="Database name"
        />
      </n-form-item>

      <template v-if="config.database.type === 'sqlite'">
        <n-form-item label="Database File:" path="dbFile">
          <n-input
            v-model:value="config.database.dbFile"
            placeholder="Path to database file"
          />
        </n-form-item>
      </template>

      <n-form-item label="Table Prefix:" path="tablePrefix">
        <n-input
          v-model:value="config.database.tablePrefix"
          placeholder="Optional table prefix"
        />
      </n-form-item>

      <template v-if="config.database.type !== 'sqlite'">
        <n-form-item label="SSL Mode:" path="sslMode">
          <n-select
            v-model:value="config.database.sslMode"
            :options="[
              { label: 'Disable', value: 'disable' },
              { label: 'Require', value: 'require' },
              { label: 'Verify Full', value: 'verify-full' }
            ]"
          />
        </n-form-item>
        <n-form-item label="DSN:" path="dsn">
          <n-input
            v-model:value="config.database.dsn"
            placeholder="Optional DSN string"
          />
        </n-form-item>
      </template>
    </n-form>
  </div>
</template>

<script setup lang="ts">
import { defineProps, computed } from "vue";
import { NForm, NFormItem, NSelect, NInput } from 'naive-ui';

type DatabaseType = 'mysql' | 'postgres' | 'sqlite';
type SSLMode = 'disable' | 'require' | 'verify-full';

interface DatabaseConfig {
  type: DatabaseType;
  host: string;
  port?: number;
  user: string;
  password: string;
  name: string;
  dbFile: string;
  tablePrefix: string;
  sslMode: SSLMode;
  dsn: string;
}

interface Config {
  database: DatabaseConfig;
}

const props = defineProps<{
  config: Config;
}>();

const defaultPort = computed(() => {
  switch (props.config.database.type) {
    case 'mysql':
      return 3306;
    case 'postgres':
      return 5432;
    default:
      return '';
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

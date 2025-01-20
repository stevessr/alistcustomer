<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { hide } from "@tauri-apps/api/app";
import Change_password from "./components/change_password.vue";

declare global {
  interface Window {
    $notification?: {
      error: (options: {
        content: string;
        duration: number;
        keepAliveOnHover: boolean;
      }) => void;
    };
  }
}

const status = ref<AlistStatus>({ running: false, pid: null });
const message = ref("");
const proxyUrl = ref(""); // 用于存储用户输入的代理URL
const proxyUsername = ref(""); // 用于存储代理用户名
const proxyPassword = ref(""); // 用于存储代理密码
const showOptions = ref(false); // 控制可选参数菜单的显示
const useProxy = ref(false); // 控制是否使用代理
const showVersionDialog = ref(false); // 控制版本信息对话框的显示
const versionInfo = ref<AlistVersionInfo | null>(null); // 存储版本信息

async function deleteDataFolder() {
  try {
    await invoke("delete_data_folder");
    console.log("Data folder deleted successfully");
  } catch (error) {
    console.error("Failed to delete data folder:", error);
  }
}

async function getAlistStatus() {
  try {
    status.value = await invoke("get_alist_status");
    message.value = "状态获取成功！";
  } catch (error) {
    message.value = `获取状态失败：${error}`;
  }
}

async function startAlist() {
  try {
    status.value = await invoke("start_alist");
    message.value = "alist 启动成功！";
  } catch (error) {
    message.value = `启动失败：${error}`;
  }
}

async function stopAlist() {
  try {
    status.value = await invoke("stop_alist");
    message.value = "alist 已停止！";
  } catch (error) {
    message.value = `停止失败：${error}`;
  }
}

async function downloadAlist() {
  try {
    const options = {
      proxyUrl: useProxy.value ? proxyUrl.value : null, // 根据是否使用代理传递URL
      proxyUsername: useProxy.value ? proxyUsername.value : null, // 传递代理用户名
      proxyPassword: useProxy.value ? proxyPassword.value : null, // 传递代理密码
    };
    await invoke("download_and_extract_alist", options); // 传递可选参数
    message.value = "alist 下载并解压成功！";
  } catch (error) {
    message.value = `下载并解压失败：${error}`;
  }
}

async function getAlistVersion() {
  try {
    versionInfo.value = await invoke<AlistVersionInfo>("get_alist_version");
    showVersionDialog.value = true; // 显示版本信息对话框
  } catch (error) {
    message.value = `获取版本信息失败：${error}`;
    window.$notification?.error({
      content: `获取版本信息失败：${error}`,
      duration: 5000,
      keepAliveOnHover: true,
    });
  }
}
</script>

<template>
  <BaseLayout>
    <template #header>
      <h1>Status Page</h1>
      <p>这是状态页面</p>
    </template>

    <n-card class="status-card">
      <n-space vertical>
        <n-alert :type="status.running ? 'success' : 'error'">
          当前 alist 状态：{{ status.running ? "运行中" : "已停止" }}
          <template #icon>
            <n-icon :name="status.running ? 'checkmark-circle' : 'close-circle'" />
          </template>
        </n-alert>

        <n-descriptions label-placement="left" bordered>
          <n-descriptions-item label="进程 ID">
            {{ status.pid || "无" }}
          </n-descriptions-item>
          <n-descriptions-item label="状态信息">
            {{ message }}
          </n-descriptions-item>
          <n-descriptions-item label="版本信息" v-if="versionInfo">
            <n-space vertical>
              <n-tag type="info" size="small">核心版本: {{ versionInfo.version }}</n-tag>
              <n-tag type="info" size="small">Web版本: {{ versionInfo.web_version }}</n-tag>
              <n-tag v-if="versionInfo.build_date" type="info" size="small">
                构建日期: {{ versionInfo.build_date }}
              </n-tag>
              <n-tag v-if="versionInfo.commit_hash" type="info" size="small">
                Git提交: {{ versionInfo.commit_hash.slice(0, 7) }}
              </n-tag>
              <n-tag v-if="versionInfo.platform" type="info" size="small">
                平台: {{ versionInfo.platform }}
              </n-tag>
            </n-space>
          </n-descriptions-item>
        </n-descriptions>

        <n-space justify="center">
          <n-button-group>
            <n-button @click="getAlistStatus" type="primary">
              刷新状态
            </n-button>
            <n-button 
              @click="startAlist" 
              :disabled="status.running"
              v-if="!status.running"
              type="success"
            >
              启动 alist
            </n-button>
            <n-button 
              @click="stopAlist" 
              :disabled="!status.running"
              v-if="status.running"
              type="error"
            >
              停止 alist
            </n-button>
          </n-button-group>
        </n-space>

        <n-space justify="center" class="additional-actions">
          <n-button @click="getAlistVersion" secondary>
            刷新版本信息
          </n-button>
          <n-button @click="showOptions = true" secondary>
            可选参数
          </n-button>
          <n-button 
            @click="downloadAlist" 
            :disabled="status.running" 
            v-if="!status.running"
            secondary
          >
            下载 alist
          </n-button>
        </n-space>

        <Change_password class="change-password" />
      </n-space>
    </n-card>

    <!-- 可选参数菜单 -->
    <n-modal v-model:show="showOptions" title="可选参数">
      <n-card style="width: 400px">
        <n-space vertical>
          <n-checkbox v-model:checked="useProxy">使用代理</n-checkbox>
          <n-input
            v-model="proxyUrl"
            placeholder="请输入代理URL"
            :disabled="!useProxy"
          />
          <n-input
            v-model="proxyUsername"
            placeholder="请输入代理用户名"
            :disabled="!useProxy"
          />
          <n-input
            v-model="proxyPassword"
            placeholder="请输入代理密码"
            :disabled="!useProxy"
            type="password"
          />
          <div style="text-align: center">
            <n-button @click="showOptions = false">关闭</n-button>
            <n-button @click="deleteDataFolder">删除数据文件夹</n-button>
          </div>
        </n-space>
      </n-card>
    </n-modal>

  </BaseLayout>
</template>

<style scoped>
.status-card {
  max-width: 800px;
  margin: 0 auto;
  padding: 2rem;
}

.additional-actions {
  margin: 1rem 0;
}

.change-password {
  margin-top: 2rem;
}

@media (max-width: 768px) {
  .status-card {
    padding: 1rem;
  }
  
  .n-button-group {
    flex-wrap: wrap;
    gap: 0.5rem;
  }
  
  .additional-actions {
    flex-direction: column;
    gap: 0.5rem;
  }
}
</style>

<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import alistversion from "./alistversion.vue";

const status = ref<AlistStatus>({ running: false, pid: null });
const message = ref("");
const proxyUrl = ref(""); // 用于存储用户输入的代理URL
const proxyUsername = ref(""); // 用于存储代理用户名
const proxyPassword = ref(""); // 用于存储代理密码
const showOptions = ref(false); // 控制可选参数菜单的显示
const useProxy = ref(false); // 控制是否使用代理

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
</script>

<template>
  <div style="text-align: center;">
    <h1>Status Page</h1>
    <p>这是状态页面</p>
    <p>当前 alist 是否运行：{{ status.running ? "是" : "否" }}</p>
    <p>alist 进程 id：{{ status.pid || "无" }}</p>
    <p>{{ message }}</p>
    <n-button @click="getAlistStatus">刷新状态</n-button>
    <n-button @click="startAlist" :disabled="status.running">启动 alist</n-button>
    <n-button @click="stopAlist" :disabled="!status.running">停止 alist</n-button>
    <br/>
    <n-button @click="showOptions = true">可选参数</n-button> <!-- 新增：打开可选参数菜单的按钮 -->
    <n-button @click="downloadAlist">下载 alist</n-button>
    <br/>
    <!--<alistversion></alistversion>-->
    <alistversion></alistversion>

    <!-- 可选参数菜单 -->
    <n-modal v-model:show="showOptions" title="可选参数">
      <n-card style="width: 400px;">
        <n-space vertical>
          <n-checkbox v-model:checked="useProxy">使用代理</n-checkbox> <!-- 是否使用代理 -->
          <n-input v-model="proxyUrl" placeholder="请输入代理URL" :disabled="!useProxy" /> <!-- 代理URL输入框 -->
          <n-input v-model="proxyUsername" placeholder="请输入代理用户名" :disabled="!useProxy" /> <!-- 代理用户名输入框 -->
          <n-input v-model="proxyPassword" placeholder="请输入代理密码" :disabled="!useProxy" type="password" /> <!-- 代理密码输入框 -->
          <n-button @click="showOptions = false">关闭</n-button> <!-- 关闭菜单 -->
        </n-space>
      </n-card>
    </n-modal>
  </div>
</template>

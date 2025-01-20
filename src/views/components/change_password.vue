<template>
  <div style="text-align: center">
    <n-form @submit.prevent="handleSubmit">
      <n-form-item label="" style="text-align: center">
        <n-input
          v-model:value="newPassword"
          type="password"
          placeholder="请输入新密码"
        />
      </n-form-item>
      <n-button type="primary" html-type="submit">更改密码</n-button>
    </n-form>
  </div>
</template>

<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";

const newPassword = ref("");

const handleSubmit = async () => {
  try {
    await invoke("set_alist_password", { newPassword: newPassword.value });
    alert("密码更改成功！");
  } catch (error) {
    alert(`密码更改失败：${error}`);
  }
};
</script>

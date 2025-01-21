<template>
  <n-split :default-size="0.8">
    <template #1>
      <n-menu
        v-model:value="activeKey"
        mode="horizontal"
        :options="menuOptions"
        responsive
      />
    </template>
  </n-split>
</template>

<script lang="ts">
import type { MenuOption } from "naive-ui";
import type { Component } from "vue";
import {
  BookOutline as BookIcon,
  PersonOutline as PersonIcon,
  WineOutline as WineIcon,
  Home as Home,
  Attach as Attach,
} from "@vicons/ionicons5";
import { Question as Question } from "@vicons/fa";
import { Status24Regular as status } from "@vicons/fluent";
import { NButton, NIcon } from "naive-ui";
import { defineComponent, h, ref } from "vue";
import { WebviewWindow } from "@tauri-apps/api/webviewWindow";

function renderIcon(icon: Component) {
  return () => h(NIcon, null, { default: () => h(icon) });
}

const menuOptions: MenuOption[] = [
  {
    label: () =>
      h(
        "a",
        {
          href: "/",
          target: "_self",
          rel: "noopenner noreferrer",
          onClick: () => {
            console.log("clicked home");
          },
        },
        "主页"
      ),
    key: "home",
    icon: renderIcon(Home),
  },
  {
    label: () =>
      h(
        "a",
        {
          href: "/about",
          target: "_self",
          rel: "noopenner noreferrer",
          onClick: () => {
            console.log("clicked about");
          },
        },
        "关于"
      ),
    key: "about",
    icon: renderIcon(Question),
    disabled: false,
  },
  {
    label: () =>
      h(
        "a",
        {
          href: "/alist",
          target: "_self",
          rel: "noopenner noreferrer",
          onClick: () => {
            console.log("clicked alist");
          },
        },
        "本地alist界面"
      ),
    key: "alist",
    icon: renderIcon(Attach),
    disabled: false,
  },
  {
    label: () =>
      h(
        "a",
        {
          href: "/status",
          target: "_self",
          rel: "noopenner noreferrer",
          onClick: () => {
            console.log("clicked alist");
          },
        },
        "alist状态"
      ),
    key: "status",
    icon: renderIcon(status),
    disabled: false,
  },
  {
    label: () =>
      h(
        "a",
        {
          href: "/config",
          target: "_self",
          rel: "noopenner noreferrer",
          onClick: () => {
            console.log("clicked config");
          },
        },
        "配置文件编辑"
      ),
    key: "config",
    icon: renderIcon(status),
    disabled: false,
  },
  {
    label: () =>
      h(
        "a",
        {
          href: "/change-password",
          target: "_self",
          rel: "noopenner noreferrer",
          onClick: () => {
            console.log("clicked change password");
          },
        },
        "修改密码"
      ),
    key: "change-password",
    icon: renderIcon(PersonIcon),
    disabled: false,
  },
  {
    label: () =>
      h(
        NButton,
        {
          type: "tertiary",
          onClick: () => {
            console.log("clicked test");
            var webview = new WebviewWindow("test", {
              url: "https://github.com/tauri-apps/tauri",
            });
            webview.once("tauri://created", function () {
              console.log("windows created");
            });
            webview.once("tauri://error", function (e) {
              console.error(e);
            });
          },
        },
        { default: () => "Tertiary" }
      ),
    key: "test",
    icon: renderIcon(PersonIcon),
    children: [
      {
        type: "group",
        label: "测试",
        key: "windowsis",
        children: [
          {
            label: "测试者",
            key: "tester",
            icon: renderIcon(PersonIcon),
            onclick: () => {
              // Modal functionality removed
            },
          },
        ],
      },
    ],
  },
  {
    label: "舞，舞，舞",
    key: "dance-dance-dance",
    icon: renderIcon(BookIcon),
    children: [
      {
        type: "group",
        label: "人物",
        key: "people",
        children: [
          {
            label: "叙事者",
            key: "narrator",
            icon: renderIcon(PersonIcon),
          },
          {
            label: "羊男",
            key: "sheep-man",
            icon: renderIcon(PersonIcon),
          },
        ],
      },
      {
        label: "饮品",
        key: "beverage",
        icon: renderIcon(WineIcon),
        children: [
          {
            label: "威士忌",
            key: "whisky",
          },
        ],
      },
      {
        label: "食物",
        key: "food",
        children: [
          {
            label: "三明治",
            key: "sandwich",
          },
        ],
      },
      {
        label: "过去增多，未来减少",
        key: "the-past-increases-the-future-recedes",
      },
    ],
  },
];

export default defineComponent({
  setup() {
    return {
      activeKey: ref<string | null>(null),
      menuOptions
    };
  },
});
</script>

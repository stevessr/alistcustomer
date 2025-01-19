// src/router/index.js
import { createRouter, createWebHistory } from 'vue-router';

// 定义路由规则
const routes = [
  {
    path: '/',
    name: 'Home',
    component: () => import('../views/Home.vue'), // 懒加载组件
  },
  {
    path: '/about',
    name: 'About',
    component: () => import('../views/About.vue'),
  },
  {
    path: '/alist',
    name: 'alist',
    component: () => import('../views/Alist.vue'),
  },
  {
    path: '/status',
    name: 'status',
    component: () => import('../views/Status.vue'),
  },
];

// 创建路由实例
const router = createRouter({
  history: createWebHistory(), // 使用 HTML5 模式的历史记录
  routes,
});

export default router;

import { createApp } from 'vue'
import App from './App.vue'
import router from './router'
import i18n from './locales'

import './assets/tailwind.css'
import { devtools } from "@vue/devtools";
if (process.env.NODE_ENV === "development") {
    devtools.connect();
  }

const app = createApp(App)

app.use(router)
app.use(i18n)

app.mount('#app')

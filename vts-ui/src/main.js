import { createApp } from 'vue'
import App from './App.vue'
import router from './router'

import '@fontsource/roboto'
import '@fontsource/roboto/900.css'
import './assets/main.css'

const app = createApp(App)
app.use(router)
app.mount('#app')

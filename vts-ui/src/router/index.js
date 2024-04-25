import { createRouter, createWebHistory } from 'vue-router'
import { initAuthClient } from '@/icp'

import FirmwaresView from '@/views/FirmwaresView.vue'

const router = createRouter({
  history: createWebHistory(),
  routes: [
    {
      path: '/',
      name: 'home',
      redirect: { name: 'firmwares' },
    },
    {
      path: '/firmwares/',
      name: 'firmwares',
      component: FirmwaresView,
    },
  ],
})
router.beforeEach(async (to, from, next) => {
  await initAuthClient()
  next()
})

export default router

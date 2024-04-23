import { createRouter, createWebHistory } from 'vue-router'
import { auth } from '../auth'

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
  await auth()
  next()
})

export default router

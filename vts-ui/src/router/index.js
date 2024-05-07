import { createRouter, createWebHistory } from 'vue-router'
import { initAuthClient } from '@/icp'

import { AgreementFirmwaresRouteName, AgreementsRouteName } from '@/constants'

import FirmwaresView from '@/views/FirmwaresView.vue'
import AgreementsView from '@/views/AgreementsView.vue'
import CreateAgreementView from '@/views/CreateAgreementView.vue'

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
    {
      path: '/firmwares/:agreement',
      name: AgreementFirmwaresRouteName,
      component: FirmwaresView,
    },
    {
      path: '/agreements/',
      name: AgreementsRouteName,
      component: AgreementsView,
    },
    {
      path: '/vehicle/link/:vehicle',
      name: 'vehicleLink',
      component: AgreementsView,
    },
    {
      path: '/agreements/create',
      name: 'createAgreement',
      component: CreateAgreementView,
    },
  ],
})
router.beforeEach(async (to, from, next) => {
  await initAuthClient()
  next()
})

export default router

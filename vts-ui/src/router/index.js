import { createRouter, createWebHistory } from 'vue-router'
import { initAuthClient } from '@/icp'

import {
  FirmwaresRouteName,
  AgreementFirmwaresRouteName,
  AgreementsRouteName,
  VehicleLinkRouteName,
  CreateAgreementRouteName,
} from '@/constants'

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
      name: FirmwaresRouteName,
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
      name: VehicleLinkRouteName,
      component: AgreementsView,
    },
    {
      path: '/agreements/create',
      name: CreateAgreementRouteName,
      component: CreateAgreementView,
    },
  ],
})
router.beforeEach(async (to, from, next) => {
  await initAuthClient()
  next()
})

export default router

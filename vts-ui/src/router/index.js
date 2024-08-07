import { createRouter, createWebHistory } from 'vue-router'
import { initAuthClient } from '@/icp'

import {
  FirmwaresRouteName,
  AgreementFirmwaresRouteName,
  AgreementsRouteName,
  VehicleLinkRouteName,
  CreateAgreementRouteName,
  VehicleRouteName,
  InvoicesRouteName,
  InvoiceRouteName,
} from '@/constants'

import FirmwaresView from '@/views/FirmwaresView.vue'
import AgreementsView from '@/views/AgreementsView.vue'
import CreateAgreementView from '@/views/CreateAgreementView.vue'
import VehicleView from '@/views/VehicleView.vue'
import InvoicesView from '@/views/InvoicesView.vue'
import InvoiceView from '@/views/InvoiceView.vue'

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
    {
      path: '/vehicles/:vehicle',
      name: VehicleRouteName,
      component: VehicleView,
    },
    {
      path: '/vehicles/:vehicle/invoices',
      name: InvoicesRouteName,
      component: InvoicesView,
    },
    {
      path: '/vehicles/:vehicle/invoices/:id',
      name: InvoiceRouteName,
      component: InvoiceView,
    },
  ],
})
router.beforeEach(async (to, from, next) => {
  const client = await initAuthClient()
  console.log(client._identity._principal.toText())
  next()
})

export default router

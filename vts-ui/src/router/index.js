import { createRouter, createWebHistory } from 'vue-router'
import { auth } from '../auth'

import VehiclesView from '@/views/VehiclesView.vue'
import GetVehicle from '@/views/GetVehicle.vue'
import CreateAgreement from '@/views/CreateAgreement.vue'
import SignAgreement from '@/views/SignAgreement.vue'
import GetAgreement from '@/views/GetAgreement.vue'
import AgreementsView from '@/views/AgreementsView.vue'

const router = createRouter({
  history: createWebHistory(),
  routes: [
    {
      path: '/',
      name: 'home',
      redirect: { name: 'vehicles' },
    },
    {
      path: '/vehicles',
      name: 'vehicles',
      component: VehiclesView,
    },
    {
      path: '/vehicles/:publicKey',
      name: 'vehicle',
      component: GetVehicle,
    },
    {
      path: '/agreements/create',
      name: 'createAgreement',
      component: CreateAgreement,
    },
    {
      path: '/agreements/sign',
      name: 'signAgreement',
      component: SignAgreement,
    },
    {
      path: '/agreements/:publicKey',
      name: 'getAgreement',
      component: GetAgreement,
    },
    {
      path: '/agreements',
      name: 'agreements',
      component: AgreementsView,
    },
  ],
})
router.beforeEach(async (to, from, next) => {
  await auth()
  next()
})

export default router

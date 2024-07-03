<script setup>
import { RouterLink, RouterView } from 'vue-router'
import { initAuthClient, initVTSClient, initICPLedgerClient } from '@/icp'
</script>
<script>
import router from '@/router'
import { Principal } from '@dfinity/principal'
import { TokenMultiplier } from '@/constants'

export default {
  data() {
    return {
      authClient: null,
      principal: null,
      balance: 0,
      email: '<email>',
    }
  },
  async beforeMount() {
    router.push({ path: window.location.pathname })
    await this.initAuthClient()
    Promise.all([this.getBalance(), this.getUser()])
  },
  methods: {
    async getBalance() {
      const icpLedgerClient = await initICPLedgerClient()

      const rawBalance = await icpLedgerClient.icrc1_balance_of({
        owner: this.principal,
        subaccount: [],
      })
      const balance = Number(rawBalance) / TokenMultiplier
      this.balance = balance

      // It is a hack right now to be able to pay for the invoice.
      console.log(
        await icpLedgerClient.icrc2_approve({
          from: { owner: this.principal, subaccount: [] },
          spender: {
            owner: Principal.fromText(import.meta.env.VITE_VTS_CANISTER_ID),
            subaccount: [],
          },
          amount: 100_000_000_000,
          fee: [],
          memo: [],
          from_subaccount: [],
          created_at_time: [],
          expected_allowance: [],
          expires_at: [],
        }),
      )
    },
    async getUser() {
      const vtsClient = await initVTSClient()
      const user = await vtsClient.get_user()
      if (user.Ok.email.length !== 0) this.email = user.Ok.email[0]
    },
    async initAuthClient() {
      this.authClient = await initAuthClient()
      this.principal = this.authClient.getIdentity().getPrincipal()
    },
    async logout() {
      await this.authClient.logout()
      alert('You are successfully logout. Please login again :)')
      await this.initAuthClient()
    },
    copyIdentity() {
      navigator.clipboard.writeText(this.principal)
      alert('Principal copied to the clipboard.')
    },
  },
}
</script>

<template>
  <header v-if="principal !== null">
    <nav>
      <a href="/">
        <img class="logo" alt="Staex logo" src="/favicon.svg" />
      </a>
      <ul>
        <li>
          <RouterLink to="/firmwares">Firmwares</RouterLink>
          <RouterLink to="/agreements">Agreements</RouterLink>
        </li>
        <li class="mouse-pointer" @click="logout">
          <!-- We need tag <a> to make it style like other menu entities. -->
          <a style="padding-right: 0">
            Logout ({{ principal.toText().slice(0, 5) }}..{{ principal.toText().slice(60) }})
            &nbsp;({{ balance }} ICP) &nbsp;({{ email }})
          </a>
        </li>
        <li class="mouse-pointer" style="margin-left: 0" @click="copyIdentity">
          <img src="/copy.svg" style="width: 1.2em" />
        </li>
      </ul>
    </nav>
  </header>
  <RouterView />
</template>

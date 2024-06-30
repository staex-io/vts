<script setup>
import { RouterLink, RouterView } from 'vue-router'
import { initAuthClient } from '@/icp'
import { ICPLedgerClient, principalToAccountId } from '@/icp_ledger'
</script>
<script>
import router from '@/router'
export default {
  data() {
    return {
      authClient: null,
      principal: '',
      balance: 0,
    }
  },
  async beforeMount() {
    router.push({ path: window.location.pathname })
    await this.initAuthClient()
    const icpLedgerClient = await ICPLedgerClient(this.authClient)
    const accountId = principalToAccountId(this.principal)
    const rawBalance = await icpLedgerClient.accountBalance({
      accountIdentifier: accountId.toHex(),
      certified: false,
    })
    const balance = Number(rawBalance) / 100000000
    this.balance = balance
  },
  methods: {
    async initAuthClient() {
      this.authClient = await initAuthClient()
      this.principal = this.authClient.getIdentity().getPrincipal().toText()
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
  <header>
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
            Logout ({{ principal.slice(0, 5) }}..{{ principal.slice(60) }})&nbsp;({{ balance }} ICP)
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

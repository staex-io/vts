<script setup>
import { RouterLink, RouterView } from 'vue-router'
import { initAuthClient } from '@/icp'
</script>
<script>
import router from '@/router'
export default {
  data() {
    return {
      authClient: null,
      principal: '',
    }
  },
  methods: {
    async logout() {
      await this.authClient.logout()
      alert('You are successfully logout. Please login again :)')
      location.reload()
    },
  },
  async beforeMount() {
    router.push({ path: window.location.pathname })
    this.authClient = await initAuthClient()
    this.principal = this.authClient.getIdentity().getPrincipal().toText()
  },
}
</script>

<template>
  <header>
    <nav>
      <a href="/">
        <img class="logo" alt="Staex logo" src="@/assets/logo-light.svg" />
      </a>
      <ul>
        <li>
          <RouterLink to="/firmwares">Firmwares</RouterLink>
        </li>
        <li class="mouse-pointer" @click="logout()">
          <!-- We need tag <a> to make it style like other manu entities. -->
          <a>Logout ({{ principal.slice(0, 5) }}..{{ principal.slice(60) }})</a>
        </li>
      </ul>
    </nav>
  </header>
  <RouterView />
</template>

<script setup>
import { RouterLink, RouterView } from 'vue-router'
</script>
<script>
import router from '@/router'
export default {
  beforeMount() {
    router.push({ path: window.location.pathname })
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
          <RouterLink to="/vehicles">Vehicles</RouterLink>
        </li>
        <li class="menu-item">
          <RouterLink to="/">Agreements</RouterLink>
          <ul class="submenu">
            <li>
              <RouterLink to="/agreements/create">Create Agreement</RouterLink>
            </li>
            <li>
              <RouterLink to="/agreements/sign">Sign Agreement</RouterLink>
            </li>
            <li>
              <RouterLink to="/agreements">Agreements</RouterLink>
            </li>
          </ul>
        </li>
      </ul>
    </nav>
  </header>
  <RouterView />
</template>

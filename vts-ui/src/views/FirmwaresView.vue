<script>
import { initVTSClient } from '@/icp'

export default {
  data() {
    return {
      loading: false,
      success: '',
      error: '',
    }
  },
  methods: {
    cleanState() {
      this.success = ''
      this.error = ''
    },
    async request() {
      if (this.loading) return
      this.loading = true
      this.cleanState()

      const vtsClient = await initVTSClient()
      const res = await vtsClient.request_firmware()
      if (res.Ok === null) this.success = 'Successfully requested new firmware!'
      if (res.Err && res.Err.AlreadyExists === null)
        this.error = 'You already have active firmware request.'

      this.loading = false
    },
  },
}
</script>

<template>
  <h1>Firmwares</h1>
  <button
    type="button"
    @click="request"
  >
    <span v-if="!loading">Request</span>
    <div
      v-if="loading"
      class="loader"
    />
  </button>
  <div>
    <p
      v-if="success !== ''"
      class="success alert"
    >
      {{ success }}
    </p>
  </div>
  <div>
    <p
      v-if="error !== ''"
      class="error alert"
    >
      {{ error }}
    </p>
  </div>
</template>

<style scoped>
.alert {
  margin-top: 20px;
}
</style>

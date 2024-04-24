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
  <button type="button" @click="request">
    <span v-if="!loading">Request</span>
    <div class="loader" v-if="loading"></div>
  </button>
  <div>
    <p class="success alert" v-if="success !== ''">{{ success }}</p>
  </div>
  <div>
    <p class="error alert" v-if="error !== ''">{{ error }}</p>
  </div>
</template>

<style scoped></style>

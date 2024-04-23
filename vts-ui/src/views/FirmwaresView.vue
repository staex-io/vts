<script>
import { initVTSClient } from '@/icp'

export default {
  data() {
    return {
      loading: false,
      success: '',
      error: ''
    }
  },
  methods: {
    async request() {
      if (this.loading) {
        console.log("..")
        return
      }
      this.loading = true;
      this.success = '';
      this.error = '';
      const vtsClient = await initVTSClient()
      console.log(vtsClient)
      console.log(vtsClient.request_firmware)
      const res = await vtsClient.request_firmware()
      console.log(res)
      console.log(res.Ok)
      if (res.Ok === null) this.success = "Successfully requested new firmware!";
      if (res.Err && res.Err.AlreadyExists === null) this.error = 'You already have active firmware request.';
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

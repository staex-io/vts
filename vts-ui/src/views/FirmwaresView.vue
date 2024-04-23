<script>
import { initVTSClient } from '@/icp'

export default {
  methods: {
    async request() {
      const vtsClient = await initVTSClient()
      const res = await vtsClient.request_firmware()
    },
  },
}
</script>

<template>
  <h1>Firmwares</h1>
  <button type="button" @click="request">Request</button>
</template>

<style scoped></style>

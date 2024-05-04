<script>
import { initVTSClient } from '@/icp'

export default {
  data() {
    return {
      fetchActiveLoader: false,
      activeText: '',

      requestNewLoader: false,

      successText: '',
      errorText: '',
    }
  },
  async beforeMount() {
    this.fetchActiveLoader = true
    const vtsClient = await initVTSClient()
    let requests = await vtsClient.get_firmware_requests_by_user()
    if (requests.Ok === null)
      this.activeText =
        'You have active firmware request. Please wait while Staex gateway is build new firmware.'
    this.fetchActiveLoader = false
  },
  methods: {
    cleanState() {
      this.successText = ''
      this.errorText = ''
    },
    async request() {
      if (this.requestNewLoader) return
      this.requestNewLoader = true
      this.cleanState()

      const vtsClient = await initVTSClient()
      const res = await vtsClient.request_firmware()
      if (res.Ok === null) this.successText = 'Successfully requested new firmware!'
      if (res.Err && res.Err.AlreadyExists === null)
        this.errorText = 'You already have active firmware request.'

      this.requestNewLoader = false
    },
  },
}
</script>

<template>
  <h1>Firmwares</h1>
  <div
    v-if="fetchActiveLoader"
    class="warning alert loader-container"
  >
    <div class="loader" />
    Fetching active firmware status...
  </div>
  <div>
    <p
      v-if="activeText !== ''"
      class="warning alert"
    >
      {{ activeText }}
    </p>
  </div>
  <button
    type="button"
    @click="request"
  >
    <span v-if="!requestNewLoader">Request new firmware</span>
    <div
      v-if="requestNewLoader"
      class="loader"
    />
  </button>
  <div
    v-if="successText !== ''"
    class="success alert"
  >
    {{ successText }}
  </div>
  <div
    v-if="errorText !== ''"
    class="error alert"
  >
    {{ errorText }}
  </div>
</template>

<style scoped>
.alert {
  margin: 20px 0 20px 0;
}
</style>

<script>
import { initVTSClient } from '@/icp'

export default {
  data() {
    return {
      fetchUserLoader: false,
      activeRequestText: '',
      vehicles: [],

      requestNewLoader: false,

      successText: '',
      errorText: '',
    }
  },
  async beforeMount() {
    this.fetchUserLoader = true

    const vtsClient = await initVTSClient()

    const requests = await vtsClient.get_firmware_requests_by_user()
    if (requests.Ok === null)
      this.activeRequestText =
        'You have active firmware request. Please wait while Staex gateway is build new firmware.'

    const user = await vtsClient.get_user()
    if (user.Ok !== undefined) this.vehicles = user.Ok.vehicles

    this.fetchUserLoader = false
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
    v-if="fetchUserLoader"
    class="warning alert loader-container"
  >
    <div class="loader" />
    Fetching active firmware status...
  </div>
  <div>
    <p
      v-if="activeRequestText !== ''"
      class="warning alert"
    >
      {{ activeRequestText }}
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

  <div v-if="!fetchUserLoader && vehicles.length">
    <h2>Available firmwares</h2>
    <table>
      <thead>
        <tr>
          <th>Internet Identity</th>
        </tr>
      </thead>
      <tbody>
        <tr
          v-for="[principal, _] in vehicles"
          :key="principal"
          class="mouse-pointer"
        >
          <td>{{ principal }}</td>
        </tr>
      </tbody>
    </table>
  </div>
  <p v-else>
    There are no vehicles at the moment.
  </p>

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

button {
  margin-bottom: 25px;
}

h2 {
  margin-bottom: 25px;
}
</style>

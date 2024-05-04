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

    for (let i = 0; i < this.vehicles.length; i++) {
      const vehicle = (await vtsClient.get_vehicle(this.vehicles[i][0])).Ok
      this.vehicles[i] = vehicle
    }

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
    async downloadFirmware(identity, arch, firmware) {
      const firmwareUrl = URL.createObjectURL(
        new Blob([new Uint8Array(firmware).buffer], { type: 'application/zip' }),
      )
      const link = document.createElement('a')
      link.href = firmwareUrl
      link.download = `${identity}.firmware.${arch}.zip`
      document.body.appendChild(link)
      link.dispatchEvent(
        new MouseEvent('click', {
          bubbles: true,
          cancelable: true,
          view: window,
        }),
      )
      document.body.removeChild(link)
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
    style="margin-bottom: 25px"
    @click="request"
  >
    <span v-if="!requestNewLoader">Request new firmware</span>
    <div
      v-if="requestNewLoader"
      class="loader"
    />
  </button>

  <div v-if="!fetchUserLoader && vehicles.length">
    <h2 style="margin-bottom: 25px">
      Available firmwares
    </h2>
    <table>
      <thead>
        <tr>
          <th>Internet Identity</th>
          <th>Arch</th>
          <th />
        </tr>
      </thead>
      <tbody>
        <tr
          v-for="{ identity, arch, firmware } in vehicles"
          :key="identity"
        >
          <td>{{ identity.toString() }}</td>
          <td>{{ arch }}</td>
          <td style="text-align: center">
            <button
              class="download-btn"
              @click="() => downloadFirmware(identity, arch, firmware)"
            >
              Download
            </button>
          </td>
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

.download-btn {
  margin: 5px;
  padding: 2px 25px 2px 25px;
}

.download-btn:hover {
  background-color: black;
}
</style>

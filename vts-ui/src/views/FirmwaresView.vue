<script>
import router from '@/router'
import { initVTSClient } from '@/icp'
import { AgreementFirmwaresRouteName, VehicleLinkRouteName } from '@/constants'

export default {
  async beforeRouteLeave(to, from) {
    const vtsClient = await initVTSClient()
    if (from.name === AgreementFirmwaresRouteName) {
      this.agreementId = 0
      const rawVehicles = await this.fetchUserVehicles(vtsClient)
      await this.prepareVehicles(vtsClient, rawVehicles)
    }
  },
  data() {
    return {
      fetchUserLoader: false,
      activeRequestText: '',
      vehicles: [],

      requestNewLoader: false,

      agreementId: 0,

      successText: '',
      errorText: '',
    }
  },
  async beforeMount() {
    this.fetchUserLoader = true

    const vtsClient = await initVTSClient()
    let rawVehicles = []

    if (this.$route.name === AgreementFirmwaresRouteName) {
      const agreementId = Number(this.$route.params.agreement)
      this.agreementId = agreementId
      const vehicles = await vtsClient.get_vehicles_by_agreement(agreementId)
      if (vehicles.Ok !== undefined) rawVehicles = vehicles.Ok
    } else {
      rawVehicles = await this.fetchUserVehicles(vtsClient)
    }

    await this.prepareVehicles(vtsClient, rawVehicles)

    this.fetchUserLoader = false
  },
  methods: {
    async prepareVehicles(vtsClient, rawVehicles) {
      for (let i = 0; i < rawVehicles.length; i++) {
        const vehicle = (await vtsClient.get_vehicle(rawVehicles[i][0])).Ok
        this.vehicles[i] = vehicle
      }
    },
    async fetchUserVehicles(vtsClient) {
      const requests = await vtsClient.get_firmware_requests_by_user()
      if (requests.Ok === null)
        this.activeRequestText =
          'You have active firmware request. Please wait while Staex gateway is build new firmware.'
      const user = await vtsClient.get_user()
      if (user.Ok !== undefined) return user.Ok.vehicles
      throw 'failed to request user vehicles'
    },
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
    linkFirmware(identity) {
      router.push({
        name: VehicleLinkRouteName,
        params: {
          vehicle: identity,
        },
      })
    },
    goToAgreement(agreement) {
      console.log(agreement)
      alert('In future redirect to the agreement will be implemented!')
    },
  },
}
</script>

<template>
  <h1>Firmwares</h1>
  <div v-if="fetchUserLoader" class="warning alert loader-container">
    <div class="loader" />
    Fetching active firmware status...
  </div>
  <div>
    <p v-if="activeRequestText !== ''" class="warning alert">
      {{ activeRequestText }}
    </p>
  </div>

  <button v-if="agreementId === 0" style="margin-bottom: 25px" @click="request">
    <span v-if="!requestNewLoader">Request new firmware</span>
    <div v-if="requestNewLoader" class="loader" />
  </button>

  <div v-if="!fetchUserLoader && vehicles.length">
    <h2 v-if="agreementId === 0" style="margin-bottom: 25px">Available firmwares</h2>
    <h2 v-if="agreementId !== 0" style="margin-bottom: 5px">
      Available firmwares for the requested agreement
    </h2>
    <button
      v-if="agreementId !== 0"
      class="action-btn"
      style="margin-bottom: 25px"
      @click="() => goToAgreement(agreementId)"
    >
      Agreement
    </button>
    <table>
      <thead>
        <tr>
          <th>Internet Identity</th>
          <th>Arch</th>
          <th />
          <th v-if="agreementId === 0" />
        </tr>
      </thead>
      <tbody>
        <tr v-for="{ agreement, identity, arch, firmware } in vehicles" :key="identity">
          <td>{{ identity.toString() }}</td>
          <td>{{ arch }}</td>
          <td style="text-align: right">
            <button class="action-btn" @click="() => downloadFirmware(identity, arch, firmware)">
              Download
            </button>
          </td>
          <td v-if="agreementId === 0" style="text-align: right">
            <button
              v-if="agreement.length === 0"
              class="action-btn"
              @click="() => linkFirmware(identity)"
            >
              Link
            </button>
            <button v-else class="action-btn" @click="() => goToAgreement(agreement)">
              Agreement
            </button>
          </td>
        </tr>
      </tbody>
    </table>
  </div>
  <p v-else>There are no vehicles at the moment.</p>

  <div v-if="successText !== ''" class="success alert">
    {{ successText }}
  </div>
  <div v-if="errorText !== ''" class="error alert">
    {{ errorText }}
  </div>
</template>

<style scoped>
.alert {
  margin: 20px 0 20px 0;
}

.action-btn {
  padding: 2px 25px 2px 25px;
}
</style>

<script>
import { initVTSClient } from '@/icp'
import { Principal } from '@dfinity/principal'
import { downloadFirmware } from '@/download_firmware'

export default {
  data() {
    return {
      vehicle: null,
    }
  },
  async beforeMount() {
    const vtsClient = await initVTSClient()
    const vehicle = this.$route.params.vehicle
    const res = await vtsClient.get_vehicle(Principal.fromText(vehicle))
    this.vehicle = res.Ok
  },
  methods: {
    publicKeyToPrincipal(publicKey) {
      return Principal.selfAuthenticating(publicKey)
    },
    goToAgreement() {
      console.log(this.vehicle.agreement)
      alert('In future redirect to the agreement will be implemented!')
    },
    downloadFirmware() {
      const identity = this.publicKeyToPrincipal(this.vehicle.public_key)
      downloadFirmware(identity, this.vehicle.arch, this.vehicle.firmware)
    },
  },
}
</script>

<template>
  <div v-if="vehicle === null" />
  <div v-else class="centered-container">
    <div class="centered-item">
      <div class="card local-row" style="width: 80%">
        <div class="card-header">Vehicle</div>
        <div class="card-content">
          <div class="card-field">
            <span class="card-field-label">Internet Identity</span>
            <span class="card-field-value">{{ publicKeyToPrincipal(vehicle.public_key) }}</span>
          </div>
          <div class="card-field">
            <span class="card-field-label">Owner</span>
            <span class="card-field-value">{{ vehicle.owner }}</span>
          </div>
          <div class="card-field">
            <span class="card-field-label">Agreement</span>
            <span class="card-field-value">
              <button class="action-btn" @click="goToAgreement">Check</button>
            </span>
          </div>
          <div class="card-field">
            <span class="card-field-label">Status</span>
            <span v-if="vehicle.on_off" class="card-field-value">On</span>
            <span v-if="!vehicle.on_off" class="card-field-value">Off</span>
          </div>
          <div class="card-field">
            <span class="card-field-label">Architecture</span>
            <span class="card-field-value">{{ vehicle.arch }}</span>
          </div>
          <div class="card-field">
            <span class="card-field-label">Firmware</span>
            <span class="card-field-value">
              <button class="action-btn" @click="downloadFirmware">Download</button>
            </span>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.action-btn {
  padding: 2px 25px 2px 25px;
}
</style>

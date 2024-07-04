<script>
import router from '@/router'
import { initVTSClient, initAuthClient } from '@/icp'
import { Principal } from '@dfinity/principal'
import { downloadFirmware } from '@/download_firmware'
import { VehicleLinkRouteName, InvoicesRouteName, monthIndexToName } from '@/constants'
import Chart from 'chart.js/auto'

export default {
  data() {
    return {
      user: null,
      vehicle: null,
      turnOnOffActive: false,
    }
  },
  watch: {
    vehicle(vehicle) {
      if (vehicle.accumulated_telemetry.length === 0) return

      const accT = vehicle.accumulated_telemetry
      const telemetryType = Object.keys(accT[0][0])[0]

      const yearly = accT[0][1].sort((a, b) => {
        if (a[0] < b[0]) return -1
        if (a[0] > b[0]) return 1
        else return 0
      })
      const years = yearly.map((year) => year[0])
      const yearsData = yearly.map((year) => Number(year[1].value))
      new Chart(document.getElementById('chart-year'), {
        type: 'bar',
        data: {
          labels: years,
          datasets: [
            {
              label: `${telemetryType} usage per year`,
              data: yearsData,
              borderWidth: 1,
              backgroundColor: 'rgb(0, 157, 196)',
            },
          ],
        },
        options: { scales: { y: { beginAtZero: true } } },
      })
      const lastYear = yearly[yearly.length - 1][0]
      const lastYearData = yearly[yearly.length - 1][1]

      const monthly = lastYearData.monthly.sort((a, b) => {
        if (a[0] < b[0]) return -1
        if (a[0] > b[0]) return 1
        else return 0
      })
      const months = monthly.map((month) => {
        const m = month[0]
        return monthIndexToName(m)
      })
      const monthlyData = monthly.map((month) => Number(month[1].value))
      new Chart(document.getElementById('chart-month'), {
        type: 'bar',
        data: {
          labels: months,
          datasets: [
            {
              label: `${telemetryType} usage per month for ${lastYear}`,
              data: monthlyData,
              borderWidth: 1,
              backgroundColor: 'rgb(0, 47, 59)',
            },
          ],
        },
        options: { scales: { y: { beginAtZero: true } } },
      })
      const lastMonth = monthly[monthly.length - 1][0]
      const lastMonthData = monthly[monthly.length - 1][1]

      const daily = lastMonthData.daily.sort((a, b) => {
        if (a[0] < b[0]) return -1
        if (a[0] > b[0]) return 1
        else return 0
      })
      const days = daily.map((day) => {
        return day[0]
      })
      const dailyData = daily.map((day) => {
        return Number(day[1])
      })
      new Chart(document.getElementById('chart-day'), {
        type: 'bar',
        data: {
          labels: days,
          datasets: [
            {
              label: `${telemetryType} usage per month for ${monthIndexToName(lastMonth)} ${lastYear}`,
              data: dailyData,
              borderWidth: 1,
              backgroundColor: 'rgb(0, 86, 104)',
            },
          ],
        },
        options: { scales: { y: { beginAtZero: true } } },
      })
    },
  },
  async beforeMount() {
    const authClient = await initAuthClient()
    this.user = authClient.getIdentity()._principal.toText()

    const vtsClient = await initVTSClient()
    const vehicle = this.$route.params.vehicle
    const res = await vtsClient.get_vehicle(Principal.fromText(vehicle))
    this.vehicle = res.Ok
  },
  methods: {
    async turnOn() {
      if (this.turnOnOffActive) return
      this.turnOnOffActive = true
      const vtsClient = await initVTSClient()
      const res = await vtsClient.turn_on_off_vehicle(
        this.publicKeyToPrincipal(this.vehicle.public_key),
        true,
      )
      if (res.Ok === null) alert('Vehicle is turned on')
      else alert('Failed to turn on vehicle')
      this.vehicle.on_off = true
      this.turnOnOffActive = false
    },
    async turnOff() {
      if (this.turnOnOffActive) return
      this.turnOnOffActive = true
      const vtsClient = await initVTSClient()
      const res = await vtsClient.turn_on_off_vehicle(
        this.publicKeyToPrincipal(this.vehicle.public_key),
        false,
      )
      if (res.Ok === null) alert('Vehicle is turned off')
      else alert('Failed to turn off vehicle')
      this.vehicle.on_off = false
      this.turnOnOffActive = false
    },
    linkFirmware() {
      router.push({
        name: VehicleLinkRouteName,
        params: {
          vehicle: this.publicKeyToPrincipal(this.vehicle.public_key),
        },
      })
    },
    goToInvoices() {
      router.push({
        name: InvoicesRouteName,
        params: {
          vehicle: this.publicKeyToPrincipal(this.vehicle.public_key),
        },
      })
    },
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
  <div v-if="vehicle !== null" class="centered-container">
    <div class="centered-item">
      <div class="card local-row" style="width: 80%">
        <div class="card-header">Vehicle</div>
        <div class="card-content">
          <div class="card-field">
            <span class="card-field-label">Internet Identity</span>
            <span class="card-field-value">{{ publicKeyToPrincipal(vehicle.public_key) }}</span>
          </div>
          <div class="card-field">
            <span class="card-field-label">Provider</span>
            <span v-if="vehicle !== null && vehicle.provider.length !== 0" class="card-field-value">
              {{ vehicle.provider[0].toText() }}
            </span>
            <span v-else class="card-field-value">
              Link vehicle to agreement to know vehicle provider
            </span>
          </div>
          <div class="card-field">
            <span class="card-field-label">Customer</span>
            <span class="card-field-value">{{ vehicle.customer }}</span>
          </div>
          <div class="card-field">
            <span class="card-field-label">Agreement</span>
            <span class="card-field-value">
              <button
                v-if="vehicle.agreement.length !== 0"
                class="action-btn"
                @click="goToAgreement"
              >
                Check
              </button>
              <button v-else class="action-btn" @click="linkFirmware">Link</button>
            </span>
          </div>
          <div class="card-field">
            <span class="card-field-label">Status</span>
            <span v-if="vehicle.on_off" class="card-field-value">
              <button disabled class="action-btn success-btn">On</button>
            </span>
            <span v-if="!vehicle.on_off" class="card-field-value">
              <button disabled class="action-btn failure-btn">Off</button>
            </span>
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
  <hr style="margin-bottom: 20px" />
  <div class="centered-container">
    <div class="item" style="width: 100%">
      <button style="width: 100%; height: 100px" @click="goToInvoices">Invoices</button>
    </div>
  </div>
  <hr style="margin: 20px 0" />
  <div
    v-if="
      vehicle !== null &&
      vehicle.provider.length !== 0 &&
      vehicle.provider[0].toText() === user &&
      !turnOnOffActive
    "
    class="centered-container"
  >
    <div class="item" style="width: 100%">
      <button style="width: 100%; height: 100px" @click="turnOn">Turn on</button>
    </div>
    <div class="item" style="width: 100%">
      <button style="width: 100%; height: 100px" @click="turnOff">Turn off</button>
    </div>
  </div>
  <div v-else-if="turnOnOffActive" class="centered-container">
    <div class="item" style="width: 100%">
      <button style="width: 100%; height: 100px">
        <div class="loader-container">
          <div class="loader" />
        </div>
      </button>
    </div>
  </div>
  <div v-else>
    <div class="alert warning" style="margin: 25px">
      <h3>In this section vehicle provider can turn on/off the vehicle</h3>
    </div>
  </div>
  <hr style="margin: 20px 0" />
  <div class="centered-container">
    <div class="centered-item">
      <div style="width: 70%">
        <h2 v-if="vehicle !== null && vehicle.accumulated_telemetry.length !== 0">
          Usage per year
        </h2>
        <canvas id="chart-year" />
        <h2 v-if="vehicle !== null && vehicle.accumulated_telemetry.length !== 0">
          Last year usage
        </h2>
        <canvas id="chart-month" />
        <h2 v-if="vehicle !== null && vehicle.accumulated_telemetry.length !== 0">
          Last month usage
        </h2>
        <canvas id="chart-day" />
      </div>
    </div>
  </div>
</template>

<style scoped>
.action-btn {
  padding: 2px 25px 2px 25px;
}
</style>

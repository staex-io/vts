<script>
import router from '@/router'
import { initVTSClient } from '@/icp'
import { Principal } from '@dfinity/principal'
import { downloadFirmware } from '@/download_firmware'
import { VehicleLinkRouteName } from '@/constants'
import Chart from 'chart.js/auto'

export default {
  data() {
    return {
      vehicle: null,
    }
  },
  watch: {
    vehicle(vehicle) {
      if (vehicle.accumulated_telemetry.length === 0) return

      const accT = vehicle.accumulated_telemetry

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
              label: `${Object.keys(accT[0][0])[0]} usage per year`,
              data: yearsData,
              borderWidth: 1,
              backgroundColor: 'rgb(0, 157, 196)',
            },
          ],
        },
        options: { scales: { y: { beginAtZero: true } } },
      })

      const monthly = accT[0][1][accT[0][1].length - 1][1].monthly.sort((a, b) => {
        if (a[0] < b[0]) return -1
        if (a[0] > b[0]) return 1
        else return 0
      })
      const months = monthly.map((month) => {
        const m = month[0]
        return this.monthIndexToName(m)
      })
      const monthlyData = monthly.map((month) => Number(month[1].value))
      new Chart(document.getElementById('chart-month'), {
        type: 'bar',
        data: {
          labels: months,
          datasets: [
            {
              label: `${Object.keys(accT[0][0])[0]} usage per month for ${accT[0][1][accT[0][1].length - 1][0]}`,
              data: monthlyData,
              borderWidth: 1,
              backgroundColor: 'rgb(0, 47, 59)',
            },
          ],
        },
        options: { scales: { y: { beginAtZero: true } } },
      })

      const daily = accT[0][1][accT[0][1].length - 1][1].monthly[0][1].daily.sort((a, b) => {
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
              label: `${Object.keys(accT[0][0])[0]} usage per month for ${this.monthIndexToName(accT[0][1][accT[0][1].length - 1][1].monthly[0][0])} ${accT[0][1][accT[0][1].length - 1][0]}`,
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
    const vtsClient = await initVTSClient()
    const vehicle = this.$route.params.vehicle
    const res = await vtsClient.get_vehicle(Principal.fromText(vehicle))
    this.vehicle = res.Ok
  },
  methods: {
    linkFirmware() {
      router.push({
        name: VehicleLinkRouteName,
        params: {
          vehicle: this.publicKeyToPrincipal(this.vehicle.public_key),
        },
      })
    },
    monthIndexToName(month) {
      const names = [
        'January',
        'February',
        'March',
        'April',
        'May',
        'June',
        'July',
        'August',
        'September',
        'October',
        'November',
        'December',
      ]
      return names[month - 1]
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
            <span class="card-field-label">Owner</span>
            <span class="card-field-value">{{ vehicle.owner }}</span>
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
  <div class="centered-container">
    <div class="centered-item">
      <div style="width: 80%">
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

<script>
import { initVTSClient, initAuthClient } from '@/icp'
import { monthIndexToName, TokenMultiplier } from '@/constants'
import { Principal } from '@dfinity/principal'
import Chart from 'chart.js/auto'

export default {
  data() {
    return {
      vehicle: '',
      ownPrincipal: '',
      invoice: null,
      agreement: null,
      activePayBtn: false,
    }
  },
  async beforeMount() {
    this.vehicle = this.$route.params.vehicle
    const invoiceId = this.$route.params.id

    const authClient = await initAuthClient()
    this.ownPrincipal = authClient.getIdentity()._principal.toText()

    const vtsClient = await initVTSClient()

    const invoiceRes = await vtsClient.get_invoice(Number(invoiceId))
    const invoice = invoiceRes.Ok

    const agreementsRes = await vtsClient.get_user_agreements()
    const agreements = agreementsRes.Ok
    this.agreement = agreements.find((agreement) => {
      return agreement.id === invoice.agreement
    })

    this.invoice = invoice

    await this.prepareChart()
  },
  methods: {
    async prepareChart() {
      const vtsClient = await initVTSClient()
      const vehicleRes = await vtsClient.get_vehicle(Principal.fromText(this.vehicle))
      const vehicle = vehicleRes.Ok

      const accT = vehicle.accumulated_telemetry
      const telemetryType = Object.keys(accT[0][0])[0]

      const yearly = accT[0][1].sort((a, b) => {
        if (a[0] < b[0]) return -1
        if (a[0] > b[0]) return 1
        else return 0
      })
      const lastYear = yearly[yearly.length - 1][0]
      const lastYearData = yearly[yearly.length - 1][1]

      const monthly = lastYearData.monthly.sort((a, b) => {
        if (a[0] < b[0]) return -1
        if (a[0] > b[0]) return 1
        else return 0
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
    prettyPeriod(period) {
      return `${period[0]} ${monthIndexToName(period[1])}`
    },
    prepareTotalCost(totalCost) {
      return Number(totalCost) / TokenMultiplier
    },
    isCustomer() {
      return this.ownPrincipal === this.agreement.vh_customer.toText()
    },
    async payForInvoice() {
      if (this.activePayBtn) return
      this.activePayBtn = true
      const vtsClient = await initVTSClient()
      await vtsClient.pay_for_invoice(this.invoice.id)
      this.invoice.status = { Paid: null }
      this.activePayBtn = false
    },
  },
}
</script>

<template>
  <div v-if="invoice !== null" class="centered-container">
    <div class="centered-item">
      <div class="card local-row" style="width: 80%">
        <div class="card-header">Invoice</div>
        <div class="card-content">
          <div class="card-field">
            <span class="card-field-label">Vehicle</span>
            <span class="card-field-value">{{ vehicle }}</span>
          </div>
          <div class="card-field">
            <span class="card-field-label">Agreement</span>
            <span class="card-field-value">{{ agreement.name }}</span>
          </div>
          <div class="card-field">
            <span class="card-field-label">Period</span>
            <span class="card-field-value">{{ prettyPeriod(invoice.period) }}</span>
          </div>
          <div class="card-field">
            <span class="card-field-label">Total cost</span>
            <span class="card-field-value">
              {{ prepareTotalCost(invoice.total_cost) }}&nbsp;ICP
            </span>
          </div>
          <div class="card-field">
            <span class="card-field-label">Status</span>
            <span class="card-field-value">
              <button v-if="invoice.status.Paid === null" class="status-btn success-btn" disabled>
                Paid
              </button>
              <button v-else class="status-btn failure-btn" disabled>Unpaid</button>
            </span>
          </div>
          <div v-if="isCustomer() && invoice.status.Unpaid === null" class="card-field">
            <span class="card-field-label">-</span>
            <span class="card-field-value">
              <button class="status-btn" @click="payForInvoice">
                <p v-if="!activePayBtn">Pay</p>
                <div v-else class="loader" />
              </button>
            </span>
          </div>
        </div>
      </div>
    </div>
  </div>
  <div class="centered-container">
    <div class="centered-item">
      <div style="width: 70%">
        <h2>Invoice usage</h2>
        <canvas id="chart-day" />
      </div>
    </div>
  </div>
</template>

<style scoped>
.status-btn {
  padding: 2px 25px 2px 25px;
}
</style>

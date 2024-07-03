<script>
import router from '@/router'
import { Principal } from '@dfinity/principal'
import { initVTSClient } from '@/icp'
import { InvoiceRouteName, monthIndexToName } from '@/constants'

export default {
  data() {
    return {
      vehicle: '',
      invoices: [],
    }
  },
  async beforeMount() {
    this.vehicle = this.$route.params.vehicle

    const vtsClient = await initVTSClient()

    const res = await vtsClient.get_vehicle(Principal.fromText(this.vehicle))
    const invoices = res.Ok.invoices

    for (let i = 0; i < invoices.length; i++) {
      const invoiceId = invoices[i]
      const res = await vtsClient.get_invoice(invoiceId)
      this.invoices.push(res.Ok)
    }
  },
  methods: {
    prettyPeriod(period) {
      return `${period[0]} ${monthIndexToName(period[1])}`
    },
    goToInvoice() {
      router.push({
        name: InvoiceRouteName,
        params: {
          vehicle: this.publicKeyToPrincipal(this.vehicle.public_key),
          id: this.invoice.id,
        },
      })
    },
  },
}
</script>

<template>
  <div class="container">
    <h1>Invoices</h1>
    <h3>Vehicle:</h3>
    <p style="margin-bottom: 25px">
      {{ vehicle }}
    </p>

    <table>
      <thead>
        <tr>
          <th>Period</th>
          <th>Total cost</th>
          <th>Status</th>
        </tr>
      </thead>
      <tbody>
        <tr v-for="{ id, period, status, total_cost } in invoices" :key="id">
          <td>{{ prettyPeriod(period) }}</td>
          <td>{{ total_cost }}&nbsp;ICP</td>
          <td>
            <button v-if="status.Paid === null" class="status-btn success-btn" disabled>
              Paid
            </button>
            <button v-if="status.Unpaid === null" class="status-btn failure-btn" disabled>
              Unpaid
            </button>
          </td>
        </tr>
      </tbody>
    </table>
  </div>
</template>

<style scoped>
.status-btn {
  padding: 2px 25px 2px 25px;
}
</style>

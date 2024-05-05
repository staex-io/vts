<script>
import { initVTSClient, initAuthClient } from '@/icp'

export default {
  data() {
    return {
      fetchAgreementsLoader: false,
      ownPrincipal: '',
      agreements: [],
    }
  },
  async beforeMount() {
    this.fetchAgreementsLoader = true

    const authClient = await initAuthClient()
    this.ownPrincipal = authClient.getIdentity()._principal.toText()

    const vtsClient = await initVTSClient()
    const agreements = await vtsClient.get_user_agreements()
    if (agreements.Ok !== undefined) this.agreements = agreements.Ok

    this.fetchAgreementsLoader = false
  },
}
</script>

<template>
  <h1>Agreements</h1>
  <div
    v-if="fetchAgreementsLoader"
    class="warning alert loader-container"
  >
    <div class="loader" />
    Fetching active agreements status...
  </div>

  <div style="margin-bottom: 25px">
    <a href="/agreements/create">Create new agreement</a>
  </div>

  <div v-if="!fetchAgreementsLoader && agreements.length">
    <h2 style="margin-bottom: 25px">
      Available agreements
    </h2>
    <table>
      <thead>
        <tr>
          <th>Name</th>
          <th>Entity</th>
          <th>Daily usage fee</th>
          <th>Gas price</th>
          <th>State</th>
        </tr>
      </thead>
      <tbody>
        <tr
          v-for="{ name, vh_provider, vh_customer, conditions, state } in agreements"
          :key="name"
        >
          <td>{{ name }}</td>
          <td>
            {{
              ownPrincipal !== vh_provider.toText() ? vh_provider.toText() : vh_customer.toText()
            }}
          </td>
          <td>{{ conditions.daily_usage_fee }}</td>
          <td>{{ conditions.gas_price }}</td>
          <td>{{ state.Unsigned === null ? 'Unsigned' : '' }}</td>
        </tr>
      </tbody>
    </table>
  </div>
  <p v-else>
    There are no agreements at the moment.
  </p>
</template>

<style scoped></style>

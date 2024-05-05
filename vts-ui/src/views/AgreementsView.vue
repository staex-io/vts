<script>
import router from '@/router'
import { initVTSClient, initAuthClient } from '@/icp'
import { Principal } from '@dfinity/principal'

export default {
  data() {
    return {
      fetchAgreementsLoader: false,
      agreements: [],
      ownPrincipal: '',

      linkVehicleLoader: '',
      vehicleToLink: '',
      linked: false,

      signAgreementLoader: '',
      signed: false,

      errorText: '',
    }
  },
  async beforeMount() {
    this.fetchAgreementsLoader = true

    const authClient = await initAuthClient()
    this.ownPrincipal = authClient.getIdentity()._principal.toText()

    const vtsClient = await initVTSClient()
    const agreements = await vtsClient.get_user_agreements()
    if (agreements.Ok !== undefined) this.agreements = agreements.Ok

    this.vehicleToLink = this.$route.params.vehicle

    this.fetchAgreementsLoader = false
  },
  methods: {
    goToCreateAgreementPage() {
      router.push({
        name: 'createAgreement',
      })
    },
    async linkVehicle(id) {
      if (this.linkVehicleLoader) return
      this.linkVehicleLoader = true
      this.errorText = ''

      if (this.$route.name === 'agreements') {
        window.location.reload()
        return
      }

      const vtsClient = await initVTSClient()
      let vehicle = Principal.fromText(this.vehicleToLink)
      let res = await vtsClient.link_vehicle(id, vehicle)
      if (res.Ok !== undefined) this.linked = true
      if (res.Err !== undefined && res.Err.AlreadyExists !== undefined) {
        this.linked = true
      } else if (res.Err !== undefined)
        this.errorText = 'Failed to link vehicle to the agreement. Try again later.'

      this.linkVehicleLoader = false
    },
    async signAgreement(id) {
      if (this.signAgreementLoader) return
      this.signAgreementLoader = true

      const vtsClient = await initVTSClient()
      const res = await vtsClient.sign_agreement(id)

      if (res.Ok !== undefined) {
        this.signed = true
      }
      if (this.$route.name === 'agreements') {
        window.location.reload()
        return
      } else if (res.Err !== undefined)
        this.errorText = 'Failed to sign the agreement. Try again later.'

      this.signAgreementLoader = false
    },
  },
}
</script>

<template>
  <h1>Agreements</h1>
  <div v-if="fetchAgreementsLoader" class="warning alert loader-container">
    <div class="loader" />
    Fetching active agreements status...
  </div>

  <div style="margin-bottom: 25px">
    <button class="mouse-pointer" @click="goToCreateAgreementPage">Create new agreement</button>
  </div>

  <div v-if="!fetchAgreementsLoader && agreements.length">
    <h2 style="margin-bottom: 25px">Available agreements</h2>
    <table>
      <thead>
        <tr>
          <th>Name</th>
          <th>Entity</th>
          <th>Daily usage fee</th>
          <th>Gas price</th>
          <th>State</th>
          <th v-if="vehicleToLink" />
        </tr>
      </thead>
      <tbody>
        <tr
          v-for="{ id, name, vh_provider, vh_customer, conditions, state } in agreements"
          :key="id"
        >
          <td>{{ name }}</td>
          <td>
            {{
              ownPrincipal !== vh_provider.toText() ? vh_provider.toText() : vh_customer.toText()
            }}
          </td>
          <td>{{ conditions.daily_usage_fee }}</td>
          <td>{{ conditions.gas_price }}</td>
          <td>
            <button
              v-if="state.Unsigned === null && ownPrincipal === vh_customer.toText()"
              class="link-btn"
              @click="() => signAgreement(id)"
              style="background-color: grey"
            >
              <span v-if="signAgreementLoaderId !== id">Sign</span>
              <div v-else class="loader" />
            </button>
            <button
              v-if="state.Unsigned === null && ownPrincipal === vh_provider.toText()"
              class="link-btn"
              style="background-color: grey"
              disabled
            >
              Unsigned
            </button>
            <button
              v-if="state.Unsigned !== null"
              class="link-btn"
              style="background-color: green"
              disabled
            >
              Signed
            </button>
          </td>
          <td v-if="vehicleToLink">
            <button v-if="!linked" class="link-btn" @click="() => linkVehicle(id)">
              <span v-if="!linkVehicleLoader">Link</span>
              <div v-if="linkVehicleLoader" class="loader" />
            </button>
            <button v-if="linked" class="link-btn" style="background-color: green">Linked</button>
          </td>
        </tr>
      </tbody>
    </table>
  </div>
  <p v-else>There are no agreements at the moment.</p>

  <div v-if="errorText !== ''" class="error alert">
    {{ errorText }}
  </div>
</template>

<style scoped>
.link-btn {
  padding: 2px 25px 2px 25px;
}

.link-btn:hover {
  background-color: black;
}

.alert {
  margin: 20px 0 20px 0;
}

.link-btn[disabled] {
  cursor: default;
}
</style>

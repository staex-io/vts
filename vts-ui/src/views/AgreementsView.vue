<script>
import router from '@/router'
import { Principal } from '@dfinity/principal'
import { initVTSClient, initAuthClient } from '@/icp'
import {
  AgreementFirmwaresRouteName,
  AgreementsRouteName,
  VehicleLinkRouteName,
  CreateAgreementRouteName,
} from '@/constants'

export default {
  beforeRouteLeave(to, from) {
    if (from.name === VehicleLinkRouteName) {
      this.vehicleToLink = ''
    }
    if (from.name === AgreementsRouteName) {
      this.vehicleToLink = to.params.vehicle
    }
  },
  data() {
    return {
      fetchAgreementsLoader: false,
      agreements: [],
      ownPrincipal: '',

      linkVehicleLoaderId: 0,
      vehicleToLink: '',
      linkedId: 0,

      signAgreementLoaderId: 0,
      signedId: 0,

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
        name: CreateAgreementRouteName,
      })
    },
    async linkVehicle(id) {
      if (this.linkedId !== 0) {
        this.errorText = 'You already linked this vehicle to an agreement.'
        return
      }
      if (this.linkVehicleLoaderId !== 0) return
      this.linkVehicleLoaderId = id
      this.errorText = ''

      if (this.$route.name === AgreementsRouteName) {
        window.location.reload()
        return
      }

      const vtsClient = await initVTSClient()
      const vehicle = Principal.fromText(this.vehicleToLink)
      const res = await vtsClient.link_vehicle(id, vehicle)
      if (res.Ok !== undefined) this.linkedId = id
      if (res.Err !== undefined && res.Err.AlreadyExists !== undefined) {
        this.linkedId = id
      } else if (res.Err !== undefined)
        if (res.Err.InvalidSigner !== undefined)
          this.errorText = 'You cannot link vehicle if you created an agreement.'
        else this.errorText = 'Failed to link vehicle to the agreement. Try again later.'

      this.linkVehicleLoaderId = 0
    },
    async signAgreement(id) {
      if (this.signAgreementLoaderId !== 0) return
      this.signAgreementLoaderId = id

      const vtsClient = await initVTSClient()
      const res = await vtsClient.sign_agreement(id)

      if (res.Ok !== undefined) this.signedId = id
      if (res.Err !== undefined) this.errorText = 'Failed to sign the agreement. Try again later.'

      this.signAgreementLoaderId = 0
    },
    goToAgreementVehicles(id) {
      router.push({
        name: AgreementFirmwaresRouteName,
        params: {
          agreement: id,
        },
      })
    },
  },
}
</script>

<template>
  <div class="container">
    <h1>Agreements</h1>
    <div v-if="fetchAgreementsLoader" class="warning alert loader-container">
      <div class="loader" />
      Fetching active agreements status...
    </div>

    <div style="margin-bottom: 25px">
      <button class="mouse-pointer" @click="goToCreateAgreementPage">Create new agreement</button>
    </div>

    <div v-if="!fetchAgreementsLoader && agreements.length">
      <h2 style="margin-bottom: 5px">Available agreements</h2>
      <p style="margin-bottom: 25px">
        <i>By pressing on agreement name you can see its vehicles</i>
      </p>
      <table>
        <thead>
          <tr>
            <th>Name</th>
            <th>Entity</th>
            <th>Daily usage fee</th>
            <th>Gas price</th>
            <th />
            <th v-if="vehicleToLink" />
          </tr>
        </thead>
        <tbody>
          <tr
            v-for="{ id, name, vh_provider, vh_customer, conditions, state } in agreements"
            :key="id"
          >
            <td class="mouse-pointer" @click="() => goToAgreementVehicles(id)">
              {{ name }}
            </td>
            <td class="mouse-pointer" @click="() => goToAgreementVehicles(id)">
              {{
                ownPrincipal !== vh_provider.toText() ? vh_provider.toText() : vh_customer.toText()
              }}
            </td>
            <td>{{ conditions.daily_usage_fee }}</td>
            <td>{{ conditions.gas_price }}</td>
            <td>
              <button
                v-if="
                  state.Unsigned === null && ownPrincipal === vh_customer.toText() && signedId != id
                "
                class="link-btn"
                @click="() => signAgreement(id)"
              >
                <span v-if="signAgreementLoaderId !== id">Sign</span>
                <div v-else class="loader" />
              </button>
              <button
                v-if="state.Unsigned === null && ownPrincipal === vh_provider.toText()"
                class="link-btn"
                disabled
              >
                Unsigned
              </button>
              <button
                v-if="state.Unsigned !== null || signedId == id"
                disabled
                class="link-btn success-btn"
              >
                Signed
              </button>
            </td>
            <td v-if="vehicleToLink">
              <button v-if="linkedId !== id" class="link-btn" @click="() => linkVehicle(id)">
                <span v-if="linkVehicleLoaderId !== id">Link</span>
                <div v-else class="loader" />
              </button>
              <button v-else class="link-btn success-btn" disabled>Linked</button>
            </td>
          </tr>
        </tbody>
      </table>
    </div>
    <p v-else>There are no agreements at the moment.</p>

    <div v-if="errorText !== ''" class="error alert">
      {{ errorText }}
    </div>
  </div>
</template>

<style scoped>
.link-btn {
  padding: 2px 25px 2px 25px;
}

.alert {
  margin: 20px 0 20px 0;
}
</style>

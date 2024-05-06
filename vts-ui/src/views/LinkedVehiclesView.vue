<script>
import { initVTSClient } from '@/icp'

export default {
  data() {
    return {
      vehicles: [],
      vehicleDetails: {},
      vehiclesByAgreement: [],
      selectedVehicle: null,
      fetchVehiclesLoader: false,
      fetchVehicleDetailsLoader: false,
      errorText: '',
    }
  },
  async beforeMount() {
    this.fetchVehiclesLoader = true
    this.errorText = ''

    const vtsClient = await initVTSClient()

    const user = await vtsClient.get_user()
    if (user.Ok !== undefined) this.vehicles = user.Ok.vehicles

    for (let i = 0; i < this.vehicles.length; i++) {
      const vehicleResponse = (await vtsClient.get_vehicle(this.vehicles[i][0])).Ok
      if (vehicleResponse) {
        this.vehicles[i] = vehicleResponse
      }
    }

    this.fetchVehiclesLoader = false
  },
  methods: {
    async viewDetails(vehicleIdentity) {
      this.fetchVehicleDetailsLoader = true
      this.errorText = ''

      const vtsClient = await initVTSClient()
      const vehicleResponse = await vtsClient.get_vehicle(vehicleIdentity)

      if (vehicleResponse.Ok !== undefined) {
        const vehicleDetails = vehicleResponse.Ok

        vehicleDetails.identity = vehicleDetails.identity.toString()
        vehicleDetails.owner = vehicleDetails.owner.toString()

        if (Array.isArray(vehicleDetails.agreement)) {
          vehicleDetails.agreement = vehicleDetails.agreement.map((agreement) =>
            agreement.toString(),
          )
        }

        this.vehicleDetails[vehicleIdentity] = vehicleDetails
        this.selectedVehicle = vehicleIdentity

        const agreementIdStr = vehicleDetails.agreement[0]
        const agreementId = BigInt(agreementIdStr)
        console.log('Agreement ID:', agreementId)
        await this.getVehiclesByAgreement(agreementId)
      } else if (vehicleResponse.Err !== undefined) {
        this.errorText = `Failed to fetch details for vehicle ${vehicleIdentity}.`
      }

      this.fetchVehicleDetailsLoader = false
    },
    async getVehiclesByAgreement(agreementId) {
      const vtsClient = await initVTSClient()
      const response = await vtsClient.get_vehicles_by_agreement(agreementId)
      console.log(response)

      if (response.Ok !== undefined) {
        this.vehiclesByAgreement = Object.keys(response.Ok).map((principal) => principal.toString())
      } else {
        this.vehiclesByAgreement = []
        this.errorText = `Failed to fetch vehicles for agreement ${agreementId}.`
      }
    },
  },
}
</script>

<template>
  <div>
    <h1>Linked Vehicles</h1>

    <div v-if="fetchVehiclesLoader" class="warning alert loader-container">
      <div class="loader" />
      Fetching linked vehicles...
    </div>

    <div v-if="!fetchVehiclesLoader && vehicles.length > 0">
      <div v-for="vehicle in vehicles" :key="vehicle.identity">
        <h2 style="margin-bottom: 25px">Available vehicles</h2>
        <table>
          <thead>
            <tr>
              <th>Vehicle Identity</th>
              <th>Actions</th>
            </tr>
          </thead>
          <tbody>
            <tr>
              <td>{{ vehicle.identity.toString() }}</td>
              <td>
                <button @click="viewDetails(vehicle.identity)">View Details</button>
              </td>
            </tr>
          </tbody>
        </table>
      </div>
    </div>

    <div v-if="selectedVehicle !== null && vehicleDetails[selectedVehicle]" class="vehicle-details">
      <h2>Vehicles by Agreement</h2>
      <ul v-if="vehiclesByAgreement.length > 0">
        <li v-for="vehicle in vehiclesByAgreement" :key="vehicle">
          {{ vehicle }}
        </li>
      </ul>
      <p v-else>No vehicles found for this agreement.</p>
      <h2>Details for Vehicle: {{ selectedVehicle }}</h2>
      <ul>
        <li v-for="(value, key) in vehicleDetails[selectedVehicle]" :key="key">
          <strong>{{ key }}:</strong> {{ value }}
        </li>
      </ul>
    </div>

    <div v-if="errorText !== ''" class="error alert">
      {{ errorText }}
    </div>
  </div>
</template>

<style scoped>
.alert {
  margin: 20px 0;
}

.loader-container {
  display: flex;
  justify-content: center;
  align-items: center;
}

.loader {
  border: 4px solid #f3f3f3;
  border-top: 4px solid #3498db;
  border-radius: 50%;
  width: 24px;
  height: 24px;
  animation: spin 1s linear infinite;
}

.vehicle-details {
  border: 1px solid #3498db;
  border-radius: 8px;
  background-color: #f9f9f9;
  padding: 15px;
  margin-top: 20px;
}
</style>

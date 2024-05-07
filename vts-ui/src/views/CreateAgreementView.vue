<script>
import router from '@/router'
import { Principal } from '@dfinity/principal'
import { initVTSClient } from '@/icp'
import { AgreementsRouteName } from '@/constants'

export default {
  data() {
    return {
      creationLoader: false,

      name: '',
      vh_customer: '',
      daily_usage_fee: 0,
      gas_price: 0,

      errorText: '',
    }
  },
  methods: {
    async createAgreement() {
      if (this.creationLoader) return
      this.creationLoader = true
      this.errorText = ''

      const vtsClient = await initVTSClient()
      const vh_customer = Principal.fromText(this.vh_customer)
      const res = await vtsClient.create_agreement(
        this.name,
        vh_customer,
        this.daily_usage_fee.toString(),
        this.gas_price.toString(),
      )
      if (res.Ok !== undefined) {
        router.push({
          name: AgreementsRouteName,
        })
      } else {
        this.errorText = 'Failed to create agreement. Try again later.'
      }

      this.creationLoader = false
    },
  },
}
</script>

<template>
  <h1>Create agreement</h1>

  <label for="name">Name</label>
  <input id="name" v-model="name" type="text" name="name" />

  <label for="vh_customer">Vehicle customer</label>
  <input id="vh_customer" v-model="vh_customer" type="text" name="vh_customer" />

  <label for="daily_usage_fee">Daily usage fee</label>
  <input id="daily_usage_fee" v-model="daily_usage_fee" type="number" name="daily_usage_fee" />

  <label for="gas_price">Gas price</label>
  <input id="gas_price" v-model="gas_price" type="number" name="gas_price" />

  <button style="margin-top: 25px; width: 100%" @click="createAgreement">
    <span v-if="!creationLoader">Create agreement</span>
    <div v-if="creationLoader" class="loader" />
  </button>

  <div v-if="errorText !== ''" class="error alert">
    {{ errorText }}
  </div>
</template>

<style scoped>
label {
  margin-top: 25px;
}

.alert {
  margin: 20px 0 20px 0;
}
</style>

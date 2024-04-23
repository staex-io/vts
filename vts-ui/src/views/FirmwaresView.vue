<script>
import { auth } from '../auth'
import { HttpAgent } from '@dfinity/agent'
import { createActor } from '/declarations/vts'

export default {
  methods: {
    async request() {
      // todo: move such init (while actor) to separate function
      const client = await auth()
      const agent = new HttpAgent({
        // todo: configure this using envs
        host: 'http://127.0.0.1:7777',
        identity: client.getIdentity(),
      })
      if (agent.isLocal()) {
        await agent.fetchRootKey()
      }
      // todo: configure this using envs
      const actor = createActor('bkyz2-fmaaa-aaaaa-qaaaq-cai', { agent })
      const res = await actor.request_firmware()
      console.log(res)
    },
  },
}
</script>

<template>
  <h1>Firmwares</h1>
  <button type="button" @click="request">Request</button>
</template>

<style scoped></style>

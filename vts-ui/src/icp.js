import { AuthClient } from '@dfinity/auth-client'
import { HttpAgent } from '@dfinity/agent'
import { createActor as createVTSActor } from '@/assets/declarations/vts'
import { createActor as createICPLedgerActor } from '@/assets/declarations/icp_ledger_canister'

export const initAuthClient = async () => {
  const authClient = await AuthClient.create()
  const isAuthenticated = await authClient.isAuthenticated()
  if (isAuthenticated) {
    return authClient
  } else {
    await new Promise((resolve) => {
      authClient.login({
        identityProvider: import.meta.env.VITE_INTERNET_IDENTITY_CANISTER_ENDPOINT,
        onSuccess: resolve,
      })
    })
    return authClient
  }
}

export const initVTSClient = async () => {
  const authClient = await initAuthClient()
  const agent = new HttpAgent({
    host: import.meta.env.VITE_ICP_NODE_ENDPOINT,
    identity: authClient.getIdentity(),
  })
  const actor = await createVTSActor(import.meta.env.VITE_VTS_CANISTER_ID, { agent })
  return actor
}

export const initICPLedgerClient = async () => {
  const authClient = await initAuthClient()
  const agent = new HttpAgent({
    host: import.meta.env.VITE_ICP_NODE_ENDPOINT,
    identity: authClient.getIdentity(),
  })
  const actor = await createICPLedgerActor(import.meta.env.VITE_ICP_LEDGER_CANISTER_ID, { agent })
  return actor
}

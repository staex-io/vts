import { AuthClient } from '@dfinity/auth-client'
import { HttpAgent } from '@dfinity/agent'
import { createActor } from '@/assets/declarations/vts'

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
  const actor = await createActor(import.meta.env.VITE_VTS_CANISTER_ID, { agent })
  return actor
}

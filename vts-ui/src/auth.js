import { AuthClient } from '@dfinity/auth-client'
export const auth = async () => {
  let authClient = await AuthClient.create()
  const isAuthenticated = await authClient.isAuthenticated()
  if (isAuthenticated) {
    return authClient
  } else {
    await new Promise((resolve) => {
      authClient.login({
        identityProvider: 'https://identity.ic0.app',
        onSuccess: resolve,
      })
    })
    return authClient
  }
}

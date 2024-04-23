import { AuthClient } from '@dfinity/auth-client'
export const auth = async () => {
  let authClient = await AuthClient.create()
  const isAuthenticated = await authClient.isAuthenticated()
  if (isAuthenticated) {
    return authClient
  } else {
    await new Promise((resolve) => {
      authClient.login({
        // configure this using envs
        identityProvider: 'http://bd3sg-teaaa-aaaaa-qaaba-cai.localhost:7777',
        onSuccess: resolve,
      })
    })
    return authClient
  }
}

import { Principal } from '@dfinity/principal'
import { createAgent } from '@dfinity/utils'
import { LedgerCanister, AccountIdentifier, SubAccount } from '@dfinity/ledger-icp'

export const ICPLedgerClient = async (authClient) => {
  const agent = await createAgent({
    identity: authClient._identity,
    host: 'http://127.0.0.1:7777',
  })
  const icpLedgerClient = LedgerCanister.create({
    agent,
    canisterId: 'ryjl3-tyaaa-aaaaa-aaaba-cai',
  })
  return icpLedgerClient
}

export const principalToAccountId = (rawPrincipal) => {
  const principal = Principal.fromText(rawPrincipal)
  const subAccount = SubAccount.fromPrincipal(principal)
  return AccountIdentifier.fromPrincipal({ principal, subAccount })
}

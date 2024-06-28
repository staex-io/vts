import { Principal } from '@dfinity/principal'
import { createAgent } from '@dfinity/utils'
import { LedgerCanister, AccountIdentifier, SubAccount } from '@dfinity/ledger-icp'

export const ICPLedgerClient = async (authClient) => {
  const agent = await createAgent({
    identity: authClient._identity,
    host: import.meta.env.VITE_ICP_NODE_ENDPOINT,
  })
  const icpLedgerClient = LedgerCanister.create({
    agent,
    canisterId: import.meta.env.VITE_ICP_LEDGER_CANISTER_ID,
  })
  return icpLedgerClient
}

export const principalToAccountId = (rawPrincipal) => {
  const principal = Principal.fromText(rawPrincipal)
  const subAccount = SubAccount.fromPrincipal(principal)
  return AccountIdentifier.fromPrincipal({ principal, subAccount })
}

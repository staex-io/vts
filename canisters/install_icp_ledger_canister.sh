#!/bin/sh

dfx canister install \
  --async-call -y icp_ledger_canister \
  --argument "
(variant {
  Init = record {
    minting_account = \"$MINTER_ACCOUNT_ID\";
    initial_values = vec {
      record {
        \"$DEFAULT_ACCOUNT_ID\";
        record {
          e8s = 10_000_000_000 : nat64;
        };
      };
    };
    send_whitelist = vec {};
    transfer_fee = opt record {
      e8s = 10_000 : nat64;
    };
    token_symbol = opt \"LICP\";
    token_name = opt \"Local ICP\";
  }
})
"

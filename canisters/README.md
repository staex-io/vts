# Canisters

## Usage

```shell
make start
# Next command in another terminal session.
make install
```

### Tests

```shell
make start
# Next commands in another terminal session.
make install
make test
```

### Setup admin and users

```shell
dfx identity get-principal
dfx canister call vts add_admin '(principal "")'
dfx canister call vts register_user '(principal "", opt "")'
```

### Predefined temeletry

In order to use predefined telemetry data use folloding command:

```shell
dfx canister call vts fill_predefined_telemetry '(principal "<vehicle provider principal>", principal "<vehicle customer principal>", "<vehicle public key in hex>")'
```

## ICP ledger canister

We need this canister to use ICP tokens and transfer them between users to pay for invoices.

Use following docs for local setup: https://internetcomputer.org/docs/current/developer-docs/defi/icp-tokens/ledger-local-setup

Use following docs to setup ICP ledger canister with icrc1 and icrc2 support: https://github.com/dfinity/examples/tree/master/rust/token_transfer_from

### Useful commands

```shell
# Check account balance.
dfx canister call icp_ledger_canister icrc1_balance_of "(record {
  owner = principal \"sgwtz-xsjjo-25gtf-i4gok-4esil-3iw33-albxt-vkzxv-iee7b-qp3d6-fqe\";
})"     


# Fill account with some tokens from treasury (faucet).
dfx canister call icp_ledger_canister icrc1_transfer "(record {
  to = record {
    owner = principal \"sgwtz-xsjjo-25gtf-i4gok-4esil-3iw33-albxt-vkzxv-iee7b-qp3d6-fqe\";
  };
  amount = 1_000_000_000;
})"

# Approve another account to use this account tokens behalf of.
dfx canister call --identity default icrc1_ledger_canister icrc2_approve "(
  record {
    spender = record {
      owner = principal \"$(dfx canister id token_transfer_from_backend)\";
    };
    amount = 10_000_000_000: nat;
  }
)"
```

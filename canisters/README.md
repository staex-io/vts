# Canisters

## Usage

```shell
make start
# Next command in another terminal session.
make install
```

Ones per terminal session before install you need to do following:

```shell
dfx identity new minter
dfx identity use minter
export MINTER_ACCOUNT_ID=$(dfx ledger account-id)
dfx identity use default
export DEFAULT_ACCOUNT_ID=$(dfx ledger account-id)
```

After that you can do installing.

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

## ICP ledger canister

We need this canister to use ICP tokens and transfer them between users to pay for invoices.

Use following docs for local setup: https://internetcomputer.org/docs/current/developer-docs/defi/icp-tokens/ledger-local-setup

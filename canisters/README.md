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
dfx canister call vts register_user '(principal "")'
```

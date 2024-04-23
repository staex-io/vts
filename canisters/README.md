# Canisters

## Usage

```shell
# In the first terminal session.
dfx start --clean
# In the second terminal session.
dfx canister create vts
dfx build
dfx canister install vts
dfx canister call vts request_firmware
```

### Tests

```shell
make start
# Next command in another terminal session.
make install
make test
```

TODO: GENERATE AND COPY DECLARATION ON BUILD TO FRONTEND ASSETS FOLDER

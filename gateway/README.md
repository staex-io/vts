# Gateway

Build and run canister before running the gateway.

```shell
cargo run
```

## Firmware signing

In order to run firmware on macOS for demo purposes it is required to sign it by self-issued certificate.

You can check [following docs](https://support.apple.com/en-gb/guide/keychain-access/kyca8916/mac) how to do it. Use name `vts-signer` for your certificate.

Commands to check that everything is fine:

```shell
# By following command you can check that your certificate in the list.
security find-identity -p codesigning
codesign -f -s "vts-signer" <some-file> --deep
```

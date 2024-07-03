fmt:
	cargo +nightly fmt

lint: fmt
	cargo clippy --tests --all-targets --all-features -- -D warnings

test: export_identity
	cargo test --jobs 1 -- --nocapture --test-threads 1 $(name)

start:
	dfx start --clean --host 127.0.0.1:7777

pre_install:
	dfx canister create --specified-id rdmx6-jaaaa-aaaaa-aaadq-cai internet_identity
	dfx canister create --specified-id ryjl3-tyaaa-aaaaa-aaaba-cai icp_ledger_canister
	dfx build internet_identity
	dfx build icp_ledger_canister
	dfx canister create --specified-id bkyz2-fmaaa-aaaaa-qaaaq-cai vts

install: pre_install update_candid update_declarations
	dfx canister install --async-call -y internet_identity
	../../install_icp_ledger_canister.sh
	dfx canister install --async-call -y vts --wasm ../../../target/wasm32-unknown-unknown/release/vts.wasm

reinstall: update_candid update_declarations
	dfx canister install --mode reinstall --async-call -y vts --wasm ../../../target/wasm32-unknown-unknown/release/vts.wasm

update_candid:
	cargo build --target wasm32-unknown-unknown --release -p vts --locked --features clean_state,predefined_telemetry
	candid-extractor ../../../target/wasm32-unknown-unknown/release/vts.wasm > vts.did

update_declarations:
	rm -rf ../../../vts-ui/src/assets/declarations
	dfx generate vts
	dfx generate icp_ledger_canister
	mv ../declarations ../../../vts-ui/src/assets

export_identity:
	dfx identity export default >../../identity.pem

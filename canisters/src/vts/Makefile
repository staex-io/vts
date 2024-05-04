fmt:
	cargo +nightly fmt

lint: fmt
	cargo clippy --tests --all-targets --all-features -- -D warnings

test: export_identity
	cargo test --jobs 1 -- --nocapture --test-threads 1 $(name)

start:
	dfx start --clean --host 127.0.0.1:7777

install: update_candid update_declarations
	dfx deploy

update_candid:
	cargo build --release --target wasm32-unknown-unknown --package vts
	candid-extractor ../../../target/wasm32-unknown-unknown/release/vts.wasm > vts.did

update_declarations:
	dfx generate vts
	rm -rf ../../../vts-ui/src/assets/declarations
	mv ../declarations ../../../vts-ui/src/assets

export_identity:
	dfx identity export default >../../identity.pem

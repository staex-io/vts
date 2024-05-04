fmt:
	cargo +nightly fmt

lint: fmt
	cargo clippy --tests --all-targets --all-features -- -D warnings

test:
	cargo test --jobs 1 -- --nocapture --test-threads 1 $(name)

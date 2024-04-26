.PHONY: lint test

bench:
	cargo bench

lint:
	cargo fmt --all -- --check
	cargo check --all-features
	cargo clippy --release -- -D warnings

test:
	cargo test --release --all

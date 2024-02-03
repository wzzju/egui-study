build:
	@RUSTFLAGS= trunk build

run:
	@cargo run

web:
	@RUSTFLAGS= trunk serve

docs: build
	@cargo doc --no-deps

clean:
	@cargo clean
	@rm -rf dist

style-check:
	@rustup component add rustfmt 2> /dev/null
	cargo fmt --all -- --check

lint:
	@rustup component add clippy 2> /dev/null
	touch src/**
	cargo clippy --all-targets --all-features -- -D warnings

.PHONY: build run web docs clean style-check lint

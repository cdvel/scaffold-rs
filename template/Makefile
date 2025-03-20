.PHONY: build test lint check bench doc clean

build:
	cargo build

test:
	cargo test --all-features

lint:
	cargo clippy --all-targets --all-features -- -D warnings

check: lint
	cargo fmt --all -- --check

bench:
	cargo bench

doc:
	cargo doc --no-deps --document-private-items --all-features --open

setup-hooks:
   pipx install pre-commit
   pre-commit install
   pre-commit run --all-files

clean:
	cargo clean

async:
	cargo build --features async

test-async:
	cargo test --features async

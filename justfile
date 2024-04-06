default: test lint format

test:
  cargo test

lint:
  cargo clippy

format:
  cargo fmt --all -- --check

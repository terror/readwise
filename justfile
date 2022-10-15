ci: build test clippy fmt-check

build:
  cargo build

clippy:
  cargo clippy --all-targets --all-features

doc:
  cargo doc --open

fmt:
  cargo +nightly fmt --all

fmt-check:
  cargo +nightly fmt --all -- --check
  @echo formatting check done

run *args:
  cargo run -- --{{args}}

test:
  cargo test

watch +COMMAND='test':
  cargo watch --clear --exec "{{COMMAND}}"

build:
    cargo build

test args:
    cargo test --{{args}}

fmt:
    cargo +nightly fmt

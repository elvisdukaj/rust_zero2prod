# CI
rustup component add llvm-tools-preview
rustup component add clippy
rustup component add rustfmt

# dev
cargo install cargo-watch
cargo install cargo-udeps

# CI
cargo install cargo-llvm-cov
cargo install cargo-audit

#  dev
cargo watch -x check -x test -x run

# CI
cargo llvm-cov
cargo clippy -- -D warnings
cargo fmt -- --check
cargo audit
set -e

echo "Checking rustfmt"
cargo fmt --all -- --check
echo "Running clippy"
cargo clippy -- -D warnings
echo "Running tests"
cargo test --

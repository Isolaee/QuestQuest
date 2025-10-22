# Auto-format Rust code and stage changes
# This script runs cargo fmt and stages the changes automatically

# Run cargo fmt
cargo fmt --all

# Stage all modified Rust files
git add -u '*.rs'

exit 0

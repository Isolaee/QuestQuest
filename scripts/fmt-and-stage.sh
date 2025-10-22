#!/bin/bash
# Auto-format Rust code and stage changes

# Run cargo fmt
cargo fmt --all

# Stage all Rust files that were modified
git add -u '*.rs'

exit 0

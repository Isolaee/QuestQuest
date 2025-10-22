# Pre-commit Hook Scripts

This directory contains scripts used by pre-commit hooks.

## fmt-and-stage.ps1 / fmt-and-stage.sh

Automatically formats Rust code using `cargo fmt --all` and stages the formatted files.

This script is called by the pre-commit hook to ensure that:
1. All Rust files are properly formatted before commit
2. Formatted changes are automatically staged
3. The commit proceeds without manual intervention

### How it works

1. Runs `cargo fmt --all` to format all Rust code
2. Runs `git add -u '*.rs'` to stage all modified Rust files
3. Returns exit code 0 to allow the commit to proceed

### Usage

This script is automatically called by pre-commit hooks. You don't need to run it manually.

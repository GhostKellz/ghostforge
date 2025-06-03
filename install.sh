#!/bin/bash
set -e

# Install ghostforge using cargo (Arch Linux)
if ! command -v cargo >/dev/null 2>&1; then
  echo "Cargo (Rust) is required. Please install Rust first." >&2
  exit 1
fi

cargo build --release
sudo install -Dm755 target/release/ghostforge /usr/bin/ghostforge

echo "ghostforge installed to /usr/bin/ghostforge"

#!/bin/bash
set -e

# Install ghostforge using cargo
if ! command -v cargo >/dev/null 2>&1; then
  echo "Cargo (Rust) is required. Please install Rust first." >&2
  exit 1
fi

cargo install --path .

# Add alias to ~/.zshrc if not present
if ! grep -q 'alias forge=' ~/.zshrc 2>/dev/null; then
  echo 'alias forge="ghostforge"' >> ~/.zshrc
  echo "Added alias 'forge' to ~/.zshrc"
else
  echo "Alias 'forge' already present in ~/.zshrc"
fi

echo "Installation complete! Open a new terminal or run 'source ~/.zshrc' to use the 'forge' command."

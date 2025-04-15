#!/usr/bin/env bash
set -e

# Only run on macOS
if [ "$(uname)" != "Darwin" ]; then
  echo "Skipping macOS dependency check on non-macOS system."
  exit 0
fi

# Check for Homebrew
if ! command -v brew >/dev/null 2>&1; then
  echo "Error: Homebrew is required on macOS but not found. Please install Homebrew (brew.sh)." >&2
  exit 1
fi

echo "Installing Homebrew dependencies via Brewfile..."
brew bundle install # Avoid potential lockfile issues

echo "Verifying project build with dependencies..."
cargo build

echo "macOS setup verification successful!" 

#!/bin/bash

set -euo pipefail

echo "🔍 Checking for dependency updates..."

# Install cargo-edit if not present
if ! command -v cargo-upgrade &> /dev/null; then
    echo "📦 Installing cargo-edit..."
    cargo install cargo-edit
fi

echo "📋 Current dependencies:"
cargo tree --depth 1

echo ""
echo "🆙 Available updates:"
cargo upgrade --dry-run

echo ""
echo "To apply updates, run: cargo upgrade"
echo "Then run: cargo update"
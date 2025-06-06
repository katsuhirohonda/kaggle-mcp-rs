#!/bin/bash

echo "Running kaggle-mcp-rs unit tests..."
echo "================================="

# Load .env if exists
if [ -f .env ]; then
    export $(cat .env | grep -v '^#' | xargs)
fi

# Run tests with output
cargo test -- --nocapture

# Run tests with coverage if tarpaulin is installed
if command -v cargo-tarpaulin &> /dev/null; then
    echo ""
    echo "Running coverage analysis..."
    cargo tarpaulin --out Xml --out Html
fi
#!/bin/bash

# Test script for kaggle-mcp-rs authentication

echo "Testing kaggle-mcp-rs authentication..."

# Build the project
cargo build || exit 1

# Create a test request
cat << EOF | cargo run 2>/dev/null
{
  "jsonrpc": "2.0",
  "method": "initialize",
  "params": {
    "protocolVersion": "2024-11-05",
    "capabilities": {},
    "clientInfo": {
      "name": "test-client",
      "version": "1.0.0"
    }
  },
  "id": 1
}
{
  "jsonrpc": "2.0",
  "method": "tools/list",
  "params": {},
  "id": 2
}
EOF

echo ""
echo "If you see the authenticate tool in the response, the server is working correctly!"
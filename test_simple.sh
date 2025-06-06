#!/bin/bash

echo "Testing initialize..."
echo '{"jsonrpc":"2.0","method":"initialize","params":{"protocolVersion":"2024-11-05","capabilities":{},"clientInfo":{"name":"test","version":"1.0"}},"id":1}' | cargo run 2>/dev/null

echo ""
echo "Building and running npx inspector..."
npx @modelcontextprotocol/inspector cargo run
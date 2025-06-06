#!/bin/bash

# Test script to verify that competitions_list tool is exposed

echo '{"jsonrpc": "2.0", "method": "initialize", "params": {"protocolVersion": "2024-11-05", "capabilities": {}, "clientInfo": {"name": "test", "version": "1.0"}}, "id": 1}' | cargo run 2>/dev/null | jq -r '.result.capabilities.tools' | grep -q 'true' && echo "✅ Server exposes tools"

echo '{"jsonrpc": "2.0", "method": "tools/list", "id": 2}' | cargo run 2>/dev/null | jq -r '.result.tools[] | select(.name == "competitions_list")' | grep -q 'competitions_list' && echo "✅ competitions_list tool is available"
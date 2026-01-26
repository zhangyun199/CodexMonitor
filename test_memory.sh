#!/bin/bash
# Test memory RPC endpoints

TOKEN="c6718ab21389231f11c9dd727c48ce3d8f6d3636e5bd0653fc48a0024aafed03"
HOST="127.0.0.1"
PORT="4732"

echo "🔐 Authenticating..."
echo "{\"jsonrpc\":\"2.0\",\"id\":1,\"method\":\"auth\",\"params\":{\"token\":\"$TOKEN\"}}" | nc $HOST $PORT

echo ""
echo "📊 Testing memory_status..."
(
  echo "{\"jsonrpc\":\"2.0\",\"id\":1,\"method\":\"auth\",\"params\":{\"token\":\"$TOKEN\"}}"
  sleep 0.5
  echo "{\"jsonrpc\":\"2.0\",\"id\":2,\"method\":\"memory_status\",\"params\":{}}"
) | nc $HOST $PORT

echo ""
echo "✅ Done! Check output above for results."

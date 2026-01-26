#!/bin/bash
# Single-shot memory_status test (auth first, then memory in one connection)
TOKEN="c6718ab21389231f11c9dd727c48ce3d8f6d3636e5bd0653fc48a0024aafed03"

node -e "
const net = require('net');
const client = new net.Socket();
let responses = [];

client.connect(4732, '127.0.0.1', () => {
  client.write('{\"jsonrpc\":\"2.0\",\"id\":\"1\",\"method\":\"auth\",\"params\":{\"token\":\"$TOKEN\"}}\n');
});

client.on('data', (data) => {
  data.toString().split('\n').filter(l => l.trim()).forEach(line => {
    responses.push(JSON.parse(line));
    console.log('Response ' + responses.length + ':', line);

    if (responses.length === 1) {
      // After auth, send memory_status
      client.write('{\"jsonrpc\":\"2.0\",\"id\":\"2\",\"method\":\"memory_status\",\"params\":{}}\n');
    }
    if (responses.length === 2) {
      client.end();
    }
  });
});

client.on('close', () => process.exit(0));
setTimeout(() => { console.log('Timeout after 5s'); process.exit(1); }, 5000);
"

#!/bin/bash

set -eu

start=$1
end=$2

parity_endpoint=${PARITY_ENDPOINT:-localhost:8541}

for i in $(seq $start $end); do
    curl -d "{\"method\":\"eth_getBlockByNumber\",\"params\":[\"$i\",true],\"id\":1,\"jsonrpc\":\"2.0\"}" \
         -H "Content-Type: application/json" \
         -X POST \
         -o "block-$i.json" \
         $parity_endpoint
done

for i in $(seq $start $end); do
    cat "block-$i.json" | \
        jq -r '[.result.timestamp, (.result.transactions | length)] | @csv'
done

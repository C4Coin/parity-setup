for i in $(seq 131 233); do curl -d "{\"method\":\"eth_getBlockByNumber\",\"params\":[\"$i\",true],\"id\":1,\"jsonrpc\":\"2.0\"}" -H "Content-Type: application/json" -X POST localhost:8541 -o "block-$i.json"; done

for i in $(seq 131 233); do cat "block-$i.json" | jq -r '[.result.timestamp, (.result.transactions | length)] | @csv'; done


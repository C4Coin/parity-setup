printf "%x" $(($(date +%s) + 300))

Two nodes (node1, node2)

password is the same as the username

node1 = 0x00aa39d30f0d20ff03a22ccfc30b7efbfca597c2
user1 = 0x00d695cd9b0ff4edc8ce55b493aec495b597e235

node2 = 0x002e28950558fbede1a9675cb113f0bd20912019
user2 = 0x001ca0bb54fcc1d736ccd820f14316dedaafd772

Recover the node account on each node, and the user accounts on both nodes:

```
curl --data '{"jsonrpc":"2.0","method":"parity_newAccountFromPhrase","params":["node1", "node1"],"id":0}' -H "Content-Type: application/json" -X POST localhost:8541
curl --data '{"jsonrpc":"2.0","method":"parity_newAccountFromPhrase","params":["user1", "user1"],"id":0}' -H "Content-Type: application/json" -X POST localhost:8541
curl --data '{"jsonrpc":"2.0","method":"parity_newAccountFromPhrase","params":["user2", "user2"],"id":0}' -H "Content-Type: application/json" -X POST localhost:8541

curl --data '{"jsonrpc":"2.0","method":"parity_newAccountFromPhrase","params":["node2", "node2"],"id":0}' -H "Content-Type: application/json" -X POST localhost:8542
curl --data '{"jsonrpc":"2.0","method":"parity_newAccountFromPhrase","params":["user1", "user1"],"id":0}' -H "Content-Type: application/json" -X POST localhost:8542
curl --data '{"jsonrpc":"2.0","method":"parity_newAccountFromPhrase","params":["user2", "user2"],"id":0}' -H "Content-Type: application/json" -X POST localhost:8542
```

Start up the nodes, then tell them about each other:

```
node1_enode=$(curl --data '{"jsonrpc":"2.0","method":"parity_enode","params":[],"id":0}' -H "Content-Type: application/json" -X POST localhost:8541 | jq -r '.result')
curl --data "{\"jsonrpc\":\"2.0\",\"method\":\"parity_addReservedPeer\",\"params\":[\"${node1_enode}\"],\"id\":0}" -H "Content-Type: application/json" -X POST localhost:8542
```

# rpc-from-user

This is split so that all transactions from user1 are sent to node1 and user2 goes to node2

```
cargo run -- --config rpc-generator-config.json --transactions 2000
jq -c 'map(select(.params[0].from != "0x001ca0bb54fcc1d736ccd820f14316dedaafd772"))' rpc.json > rpc-from-user1.json
jq -c 'map(select(.params[0].from == "0x001ca0bb54fcc1d736ccd820f14316dedaafd772"))' rpc.json > rpc-from-user2.json
```

```
curl --data @rpc-from-user1.json -H "Content-Type: application/json" -X POST localhost:8541 & curl --data @rpc-from-user2.json -H "Content-Type: application/json" -X POST localhost:8542 &
```

Analyzing shows 2000 transactions in 14 seconds (142 tx/sec)

```
"0x595029e1",443
"0x595029e5",583
"0x595029ea",625
"0x595029ef",349
```

# rpc-part

Transactions from user1 and user2 are sent to both nodes

```
cargo run -- --config rpc-generator-config.json --transactions 1000 -o rpc-part1.json
cargo run -- --config rpc-generator-config.json --transactions 1000 -o rpc-part2.json
```

```
cargo run -- --config rpc-generator-config.json --transactions 1000 -o rpc-part1.json
```

# Change in stake

Two nodes (node1, node2), no user accounts

password is the same as the username

node1 = 0x00aa39d30f0d20ff03a22ccfc30b7efbfca597c2
node2 = 0x002e28950558fbede1a9675cb113f0bd20912019

From within the `two-node/ouroboros` directory, start up the nodes with `node1-initial.toml` and `node2-initial.toml`:

```
cd two-node/ouroboros
parity --config node1-initial.toml
```

```
cd two-node/ouroboros
parity --config node2-initial.toml
```

Recover the node account on each node:

```
curl --data '{"jsonrpc":"2.0","method":"parity_newAccountFromPhrase","params":["node1", "node1"],"id":0}' -H "Content-Type: application/json" -X POST localhost:8541

curl --data '{"jsonrpc":"2.0","method":"parity_newAccountFromPhrase","params":["node2", "node2"],"id":0}' -H "Content-Type: application/json" -X POST localhost:8542
```

Start up the nodes with `node1.toml` and `node2.toml`, then tell them about each other:

```
node1_enode=$(curl --data '{"jsonrpc":"2.0","method":"parity_enode","params":[],"id":0}' -H "Content-Type: application/json" -X POST localhost:8541 | jq -r '.result')
curl --data "{\"jsonrpc\":\"2.0\",\"method\":\"parity_addReservedPeer\",\"params\":[\"${node1_enode}\"],\"id\":0}" -H "Content-Type: application/json" -X POST localhost:8542
```

## rpc-from-user

Extract only the transactions from node2 to node1 in order to change stake from being balanced to being skewed towards node1

```
cargo run --bin stake-change -- --config rpc-generator-stake-change.json
```

Send those transactions to node2:

```
curl --data @rpc.json -H "Content-Type: application/json" -X POST localhost:8542 &
```

## Configuring the EC2 instance

- c4.2xlarge
- Same availability zone
- Same subnet, if possible

Modify the security group to allow all traffic between members in the
current security group.

## Setting up the software

```
sudo yum install -y git curl gcc gcc-c++ libudev-devel openssl-devel jq tmux emacs
curl https://sh.rustup.rs -sSf | sh
export PATH=$PATH:$HOME/.cargo/bin

git clone https://github.com/integer32llc/parity-setup.git
git clone https://github.com/input-output-hk/parity.git

cd ~/parity
git checkout ouroboros
cargo build --release
```

## Setting up the experiment

Another seed may be used, just make sure all nodes are using the same
to be consistent.

```
cd ~/parity-setup/multinode
mkdir scratch
cd scratch
cargo run --release -- \
      --config ../rpc-generator-config.json \
      --transactions 100000 \
      --seed 11808094048570281606 \
      -o 100k.json
../split.sh 100k.json 1000
```

## Running the experiment

1. Initialize the node
1. Run the node
1. Request transactions

### Overview

There is one authority account and one user account per node, following the pattern of `userX` and `nodeX`. The password is the same as the username.

```
node1 = 0x00aa39d30f0d20ff03a22ccfc30b7efbfca597c2
user1 = 0x00d695cd9b0ff4edc8ce55b493aec495b597e235

node2 = 0x002e28950558fbede1a9675cb113f0bd20912019
user2 = 0x001ca0bb54fcc1d736ccd820f14316dedaafd772
```

### Initialization

Start the server with the `-initial` configuration file. Once it's running, recover the authority account on each node, then *all* of the user accounts on all nodes.

**Node1**

```
~/parity/target/release/parity --config node1-initial.toml
```

```
curl --data '{"jsonrpc":"2.0","method":"parity_newAccountFromPhrase","params":["node1", "node1"],"id":0}' -H "Content-Type: application/json" -X POST localhost:8540
curl --data '{"jsonrpc":"2.0","method":"parity_newAccountFromPhrase","params":["user1", "user1"],"id":0}' -H "Content-Type: application/json" -X POST localhost:8540
curl --data '{"jsonrpc":"2.0","method":"parity_newAccountFromPhrase","params":["user2", "user2"],"id":0}' -H "Content-Type: application/json" -X POST localhost:8540
```

**Node2**

```
~/parity/target/release/parity --config node2-initial.toml
```

```
curl --data '{"jsonrpc":"2.0","method":"parity_newAccountFromPhrase","params":["node2", "node2"],"id":0}' -H "Content-Type: application/json" -X POST localhost:8540
curl --data '{"jsonrpc":"2.0","method":"parity_newAccountFromPhrase","params":["user1", "user1"],"id":0}' -H "Content-Type: application/json" -X POST localhost:8540
curl --data '{"jsonrpc":"2.0","method":"parity_newAccountFromPhrase","params":["user2", "user2"],"id":0}' -H "Content-Type: application/json" -X POST localhost:8540
```

### Running

Start up the nodes, then tell them about each other. TODO: do peers
communicate other peers? That is, can we have nodes 2-40 just talk to
node 1?

```
cd ~/parity-setup/multinode/authority-round # or `ouroboros`
~/parity/target/release/parity --config node1.toml
```

```
node1_enode=$(curl --data '{"jsonrpc":"2.0","method":"parity_enode","params":[],"id":0}' -H "Content-Type: application/json" -X POST 172.31.14.166:8540 | jq -r '.result')
curl --data "{\"jsonrpc\":\"2.0\",\"method\":\"parity_addReservedPeer\",\"params\":[\"${node1_enode}\"],\"id\":0}" -H "Content-Type: application/json" -X POST localhost:8540
```

### Experiment

```
cd ~/parity-setup/multinode/scratch
BASENAME=100k.json ../load.sh 0 2 # offset, total nodes
BASENAME=100k.json ../load.sh 1 2 # run this on the second node
```

Find the highest block (can probably set to a Very High Number too)
then dump the statistics

```
PARITY_ENDPOINT=localhost:8540 ../../analyze-blocks.sh 1 185 | tee results.csv
```

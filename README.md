# Parity setup instructions

These instructions are modeled from [the parity demo PoA
tutorial](https://github.com/paritytech/parity/wiki/Demo-PoA-tutorial) and customized for running
the Ouroboros protocol in a variety of situations for experimental setup.

## Build Parity

- Install Rust 1.18.0 using https://rustup.rs/.
- Check out the ouroboros branch of the parity fork: https://github.com/input-output-hk/parity/tree/ouroboros
- Compile in release mode: `cargo build --release`. The resulting binary will be `target/release/parity`.

## Parity RPC generator

You will also need to `cargo build` the RPC, which makes it easier
to generate transaction JSON requests.

## Single node setup

- Start parity in order to create an account using `parity --config single-node-initial.toml`
- Create the validator/stakeholder account:

    ```
    curl --data '{"jsonrpc":"2.0","method":"parity_newAccountFromPhrase","params":["node1", "node1"],"id":0}' -H "Content-Type: application/json" -X POST localhost:8541
    ```

    This should return:

    ```

    {"jsonrpc":"2.0","result":"0x00aa39d30f0d20ff03a22ccfc30b7efbfca597c2","id":0}
    ```

- Create a non-stakeholder account:

    ```
    curl --data '{"jsonrpc":"2.0","method":"parity_newAccountFromPhrase","params":["user", "user"],"id":0}' -H "Content-Type: application/json" -X POST localhost:8541
    ```

    This should return:

    ```
    {"jsonrpc":"2.0","result":"0x004ec07d2329997267ec62b4166639513386f32e","id":0}
    ```

- If desired, modify the account balance of each account in `single-node-config.json` under the `accounts` key. The amount should be large.
- Stop parity
- Restart with `parity --config single-node.toml`, this now has `node1` configured as a signer
- Create some transactions to send by running theRPC generator with the config file `single-node-rpc-generator-config.json`. The balances in `single-node-rpc-generator-config.json`  are the initial amounts available to transfer; these amounts need to fit in a `u64`. Optionally specify the number of transactions to generate:

    ```
    parity-rpc-generator --config single-node-rpc-generator-config.json --transactions 50
    ```

- With the `rpc.json` generated, send the request:

    ```
    curl --data @/absolute/path/to/rpc.json -H "Content-Type: application/json" -X POST localhost:8541
    ```

## Viewing blocks

- In the parity logs, you should see transactions being mined, which looks like:

    ```
    2017-06-23 14:14:40  Transaction mined (hash fb8b6a122d4a2a77550ace5564fb613501e92b4f3d5e2bc1fb7bb8175f27d826)
    ```

- The number of blocks should go up. The number of the last block is near the beginning of each line. Example of mining block #7:

    ```
    2017-06-23 13:46:37  Syncing       #6 78d8…bf6f     0 blk/s // etc...
    2017-06-23 13:46:47  Syncing       #7 068b…eb29     0 blk/s // etc...
    ```

- You can see the last block that was mined by requesting it via the API:

    ```
    curl --data '{"method":"eth_getBlockByNumber","params":["latest",true],"id":1,"jsonrpc":"2.0"}' -H "Content-Type: application/json" -X POST localhost:8541
    ```


## Generating RPC requests

### Running

```
RPC generator

USAGE:
    parity-rpc-generator [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --config <FILE.json>
    -o, --output <OUTPUT>        [default: rpc.json]
        --seed <N>
        --transactions <N>       [default: 10]
```

### Config JSON example

```json
[
  {
    "id": "alpha",
    "balance": "1000000",
    "password": "hunter2"
  },
  {
    "id": "beta",
    "balance": "1",
    "password": "qwerty"
  }
]
```

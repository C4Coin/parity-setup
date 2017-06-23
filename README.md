# Parity setup instructions

These instructions are modeled from [the parity demo PoA
tutorial](https://github.com/paritytech/parity/wiki/Demo-PoA-tutorial) and customized for running
the Ouroboros protocol in a variety of situations for experimental setup.

## Parity RPC generator

You will also need to check out and `cargo build` the
[parity-rpc-generator](https://github.com/integer32llc/parity-rpc-generator), which makes it easier
to generate transaction JSON requests.

## Single node setup

- Compile parity and have the binary in your `$PATH`.
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

- Visit localhost:8081 in your browser
- Go to Accounts -> Restore and enter:
  - account recovery phrase: `node1`
  - account name: `node1`
  - password: `node1`
  - password (repeat): `node1`
- Import
- Also restore an account with `user` in all the fields instead of `node1`
- If desired, modify the account balance of each account in `single-node-config.json` under the `accounts` key. The amount should be large.
- Stop parity
- Restart with `parity --config single-node.toml`, this now has `node1` configured as a signer
- Create some transactions to send by running the
[parity-rpc-generator](https://github.com/integer32llc/parity-rpc-generator) with the config file `single-node-rpc-generator-config.json`. The balances in `single-node-rpc-generator-config.json`  are the initial amounts available to transfer; these amounts need to fit in a `u64`. Optionally specify the number of transactions to generate:

    ```
    parity-rpc-generator --config single-node-rpc-generator-config.json --transactions 50
    ```

- With the `rpc.json` generated, send the request:

    ```
    curl --data @/absolute/path/to/rpc.json -H "Content-Type: application/json" -X POST localhost:8541
    ```
## Running

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

## Config JSON example

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

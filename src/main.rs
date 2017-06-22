extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use serde::Serialize;

static JSONRPC_VERSION: &str = "2.0";
static METHOD_NAME: &str = "personal_sendTransaction";

#[derive(Debug, Clone, PartialEq, Serialize)]
struct Wrapper<P: Serialize> {
    jsonrpc: &'static str,
    method: &'static str,
    params: P,
    id: usize,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
struct PersonalSendTransactionParams {
    from: String,
    to: String,
    value: String,
}


//"user"]

//curl --data '{,"params":[,"id":0}' -H "Content-Type: application/json" -X POST localhost:8540

fn main() {
    let from = "0x004ec07d2329997267Ec62b4166639513386F32E";
    let to = "0x00Bd138aBD70e2F00903268F3Db08f2D25677C9e";
    let value = "0xde0b6b3a7640000";

    let param1 = PersonalSendTransactionParams {
        from: from.into(),
        to: to.into(),
        value: value.into(),
    };

    let param2 = "user";

    let params = (param1, param2);

    let rpc1 = Wrapper {
        jsonrpc: JSONRPC_VERSION,
        method: METHOD_NAME,
        params: params,
        id: 0,
    };

    let rpc = vec![rpc1];

    println!("{}", serde_json::to_string(&rpc).unwrap());
}

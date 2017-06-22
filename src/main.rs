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
    id: RpcId,
}

type RpcId = usize;
type PersonalSendTransaction = Wrapper<PersonalSendTransactionParams>;

impl PersonalSendTransaction {
    fn new(params: PersonalSendTransactionParams, id: RpcId) -> Self {
        Wrapper {
            jsonrpc: JSONRPC_VERSION,
            method: METHOD_NAME,
            params, id,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
struct Password(&'static str);

#[derive(Debug, Clone, PartialEq, Serialize)]
struct AccountId(String);

#[derive(Debug, Clone, PartialEq, Serialize)]
struct PersonalSendTransactionParams(Transaction, Password);

#[derive(Debug, Clone, PartialEq, Serialize)]
struct Transaction {
    from: AccountId,
    to: AccountId,
    value: String,
}

#[test]
fn like_the_wiki() {
    let from = AccountId("0x004ec07d2329997267Ec62b4166639513386F32E".into());
    let to = AccountId("0x00Bd138aBD70e2F00903268F3Db08f2D25677C9e".into());
    let value = "0xde0b6b3a7640000";

    let transaction = Transaction {
        from: from,
        to: to,
        value: value.into(),
    };

    let params = PersonalSendTransactionParams(transaction, Password("user"));

    let rpc = vec![
        PersonalSendTransaction::new(params, 0),
    ];

    let actual = serde_json::to_string(&rpc).unwrap();

    let expected = r#"[{"jsonrpc":"2.0","method":"personal_sendTransaction","params":[{"from":"0x004ec07d2329997267Ec62b4166639513386F32E","to":"0x00Bd138aBD70e2F00903268F3Db08f2D25677C9e","value":"0xde0b6b3a7640000"},"user"],"id":0}]"#;
    assert_eq!(actual, expected);
}

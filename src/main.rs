extern crate clap;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

extern crate rand;

use std::collections::HashMap;
use std::cmp::{min, max};
use std::fs::File;

use clap::{Arg, App};
use serde::Serialize;
use rand::{Rng, SeedableRng};
use rand::distributions::{IndependentSample, Range};

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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Password(String);

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
struct AccountId(String);

#[derive(Debug, Clone, PartialEq, Serialize)]
struct PersonalSendTransactionParams(Transaction, Password);

#[derive(Debug, Clone, PartialEq, Serialize)]
struct Transaction {
    from: AccountId,
    to: AccountId,
    value: String,
}

struct Account {
    id: AccountId,
    balance: u64,
}

struct TransactionGenerator<'a, R> {
    accounts: &'a mut [Account],
    range: Range<usize>,
    rng: R,
}

impl<'a, R> TransactionGenerator<'a, R> {
    fn new(accounts: &'a mut [Account], rng: R) -> Self {
        let len = accounts.len();
        Self {
            accounts,
            range: Range::new(0, len),
            rng,
        }
    }
}

impl<'a, R> Iterator for TransactionGenerator<'a, R>
where
    R: rand::Rng,
{
    type Item = (AccountId, AccountId, u64);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let a_idx = self.range.ind_sample(&mut self.rng);
            let b_idx = self.range.ind_sample(&mut self.rng);

            if a_idx == b_idx { continue }

            let lo_idx = min(a_idx, b_idx);
            let hi_idx = max(a_idx, b_idx);

            let (left, right) = self.accounts.split_at_mut(hi_idx);

            let a = &mut left[lo_idx];
            let b = &mut right[0];

            let (src, dest) = if self.rng.gen() { (a, b) } else { (b, a) };

            if src.balance == 0 { continue }
            let money = self.rng.gen_range(0, src.balance);
            if money == 0 { continue }

            src.balance -= money;
            dest.balance += money;

            return Some((src.id.clone(), dest.id.clone(), money));
        }
    }
}

//struct StringNumber(String)

#[derive(Debug, Clone, Deserialize)]
struct AccountConfig {
    id: AccountId,
    balance: String,
    password: Password,
}

fn parse_config_file(config_file: &str) -> (Vec<Account>, HashMap<AccountId, Password>) {
    let config_file = File::open(config_file).expect("Config file not found");
    let config: Vec<AccountConfig> =
        serde_json::from_reader(config_file).expect("Unable to parse config file");

    let passwords =
        config.iter()
        .map(|conf| (conf.id.clone(), conf.password.clone()))
        .collect();

    let accounts =
        config.into_iter()
        .map(|conf| {
            Account {
                id: conf.id,
                balance: conf.balance.parse().expect("Unable to parse balance"),
            }
        })
        .collect();

    (accounts, passwords)
}

fn main() {
    let matches = App::new("RPC generator")
        .arg(Arg::with_name("config")
             .long("config")
             .value_name("FILE.json")
             .takes_value(true))
        .arg(Arg::with_name("output")
             .long("output")
             .short("o")
             .value_name("OUTPUT")
             .default_value("rpc.json")
             .takes_value(true))
        .arg(Arg::with_name("transactions")
             .long("transactions")
             .value_name("N")
             .default_value("10")
             .takes_value(true))
        .arg(Arg::with_name("seed")
             .long("seed")
             .value_name("N")
             .takes_value(true))
        .get_matches();

    let config_file = matches.value_of("config").expect("Must provide config file");
    let output_file = matches.value_of("output").expect("Must provide output file");

    let count =
        matches.value_of("transactions")
        .unwrap()
        .parse()
        .expect("transactions must be a number");

    let (mut accounts, passwords) = parse_config_file(&config_file);

    let seed =
        matches.value_of("seed")
        .map(|s| s.parse().expect("Unable to parse seed"))
        .unwrap_or_else(|| rand::thread_rng().gen());

    let mut rng = rand::StdRng::from_seed(&[seed]);
    println!("Used seed {}", seed);

    let transactions: Vec<_> =
        TransactionGenerator::new(&mut accounts, &mut rng)
        .take(count)
        .enumerate()
        .map(|(id, (from, to, value))| {
            let password = passwords[&from].clone();
            let transaction = Transaction { from, to, value: format!("0x{:x}", value) };
            let params = PersonalSendTransactionParams(transaction, password);
            PersonalSendTransaction::new(params, id)
        })
        .collect();

    let output = File::create(output_file).expect("Unable to create output file");
    serde_json::to_writer(output, &transactions).expect("Unable to convert to JSON");

    println!("RPC body written to {}", output_file);
    println!("Final balances after {} transactions:", count);
    for account in &accounts {
        println!("{}:\t{}", account.id.0, account.balance);
    }
}

#[cfg(test)]
mod test {
    use super::*;

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

        let params = PersonalSendTransactionParams(transaction, Password("user".into()));

        let rpc = vec![
            PersonalSendTransaction::new(params, 0),
        ];

        let actual = serde_json::to_string(&rpc).unwrap();

        let expected = r#"[{"jsonrpc":"2.0","method":"personal_sendTransaction","params":[{"from":"0x004ec07d2329997267Ec62b4166639513386F32E","to":"0x00Bd138aBD70e2F00903268F3Db08f2D25677C9e","value":"0xde0b6b3a7640000"},"user"],"id":0}]"#;
        assert_eq!(actual, expected);
    }

    #[test]
    fn random_transactions() {
        let mut rng = rand::isaac::Isaac64Rng::from_seed(&[1,2,3,4]);

        let mut accounts = vec![
            Account {
                id: AccountId("a".into()),
                balance: 1000,
            },
            Account {
                id: AccountId("b".into()),
                balance: 1000,
            },
        ];

        let transactions: Vec<_> =
            TransactionGenerator::new(&mut accounts, &mut rng)
            .take(10)
            .collect();

        assert_eq!(
            transactions,
            [
                (AccountId("a".into()), AccountId("b".into()), 594),
                (AccountId("b".into()), AccountId("a".into()), 1300),
                (AccountId("b".into()), AccountId("a".into()), 24),
                (AccountId("a".into()), AccountId("b".into()), 1240),
                (AccountId("b".into()), AccountId("a".into()), 1443),
                (AccountId("b".into()), AccountId("a".into()), 42),
                (AccountId("a".into()), AccountId("b".into()), 1347),
                (AccountId("a".into()), AccountId("b".into()), 94),
                (AccountId("b".into()), AccountId("a".into()), 596),
                (AccountId("a".into()), AccountId("b".into()), 503),
            ]
        );
    }
}

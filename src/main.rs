// @format
use clap::{Args, Parser, Subcommand};
use dirs::home_dir;
#[allow(unused_imports)]
use ethers::{
    contract::{Eip712, EthAbiType},
    core::k256::ecdsa::SigningKey,
    core::types::{transaction::eip712::Eip712, Signature, U256},
    signers::{HDPath, Ledger, LocalWallet, Signer},
};
use serde_json::{json, Value};
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Init(InitArgs),
    Submit(SubmitArgs),
    SubmitLedger(LedgerArgs),
    Vote(VoteArgs),
    VoteLedger(VoteLedgerArgs),
}

#[derive(Args)]
struct InitArgs {
    password: Option<String>,
}

#[derive(Args)]
struct SubmitArgs {
    password: Option<String>,
    href: Option<String>,
    title: Option<String>,
}

#[derive(Args)]
struct LedgerArgs {
    href: Option<String>,
    title: Option<String>,
    address_index: Option<usize>,
}

#[derive(Args)]
struct VoteArgs {
    password: Option<String>,
    href: Option<String>,
}

#[derive(Args)]
struct VoteLedgerArgs {
    href: Option<String>,
    address_index: Option<usize>,
}


fn get_config_path() -> PathBuf {
    let mut config_dir = home_dir().unwrap();
    config_dir.push(".kiwistand");
    return config_dir;
}

fn store_key(password: &String) {
    let config_dir = get_config_path();
    let _ = fs::create_dir(&config_dir);
    let name = "key";

    let mut key_path = get_config_path();
    key_path.push(name);
    if !Path::new(&key_path).exists() {
        let mut rng = rand::thread_rng();
        LocalWallet::new_keystore(&config_dir, &mut rng, password, Some(name)).unwrap();
        return;
    }
    println!("Bailed from creating key store as it already exists.");
}

fn read_key(password: &String) -> LocalWallet {
    let mut key_path = get_config_path();
    key_path.push("key");
    let wallet = match LocalWallet::decrypt_keystore(key_path, password) {
        Ok(wallet) => wallet,
        Err(_error) => panic!("Problem reading and/or decrypting the key store"),
    };
    return wallet;
}

#[derive(Debug, Clone, Eip712, EthAbiType)]
#[eip712(
    name = "kiwinews",
    version = "1.0.0",
    salt = "kiwinews domain separator salt"
)]
pub struct Message {
    pub title: String,
    pub href: String,
    pub r#type: String,
    pub timestamp: U256,
}

fn get_unix_time() -> u64 {
    let start = SystemTime::now();
    let now = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    return now.as_secs();
}

#[cfg(test)]
mod tests {
    use super::*;
    use ethers::{core::types::H160, signers::Wallet};

    #[tokio::test]
    async fn compare_signatures() {
        let timestamp = 1676559616;
        let title = "hello world";
        let href = "https://example.com";
        let message = Message {
            title: String::from(title),
            href: String::from(href),
            r#type: String::from("amplify"),
            timestamp: U256::from(timestamp),
        };
        dbg!(&message);

        let wallet: Wallet<SigningKey> =
            "ad54bdeade5537fb0a553190159783e45d02d316a992db05cbed606d3ca36b39"
                .parse()
                .unwrap();
        let expected: H160 = "0x0f6A79A579658E401E0B81c6dde1F2cd51d97176"
            .parse()
            .unwrap();
        assert_eq!(wallet.address(), expected);
        let signature = sign(wallet, &message).await;
        assert_eq!(signature.to_string(), "1df128dfe1f86df4e20ecc6ebbd586e0ab56e3fc8d0db9210422c3c765633ad8793af68aa232cf39cc3f75ea18f03260258f7276c2e0d555f98e1cf16672dd201c");
    }
}

async fn sign_ledger(message: &Message, address_index: usize) -> Signature {
    let ledger = Ledger::new(HDPath::LedgerLive(address_index), 1u64).await.unwrap();

    return ledger
        .sign_typed_struct(message)
        .await
        .expect("failed to sign typed data");
}

async fn sign(wallet: LocalWallet, message: &Message) -> Signature {
    return wallet
        .sign_typed_data(message)
        .await
        .expect("Couldn't sign message");
}

async fn create_message(
    password: &String,
    href: &String,
    title: &String,
    ledger: bool,
    address_index: Option<usize>,
) -> Value {
        let timestamp = get_unix_time();
    let message = Message {
        title: String::from(title),
        href: String::from(href),
        r#type: String::from("amplify"),
        timestamp: U256::from(timestamp),
    };
    let sig;
    if ledger {
        let index = address_index.unwrap_or(0);
        sig = sign_ledger(&message, index).await;
    } else {
        let wallet = read_key(password);
        sig = sign(wallet, &message).await;
    }
    // TODO: We should actually test this signature against the signature
    // from JS and make sure they're equal.
    let body = json!({
        "title": message.title,
        "href": message.href,
        "type": message.r#type,
        "timestamp": timestamp,
        "signature": format!("0x{}", sig.to_string()),
    });
    return body;
}

async fn send(message: Value) {
    let client = reqwest::Client::new();
    dbg!(&message);
    let result = client
        .post("https://news.kiwistand.com/messages")
        .json(&message)
        .send()
        .await;

    let response = match result {
        Ok(response) => response,
        Err(_error) => panic!("Failed sending message"),
    };
    let body = response.text().await;
    if let Err(e) = dbg!(body) {
        eprintln!("Error: {:?}", e);
}}


#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Init(args) => {
            let password = match &args.password {
                Some(password) => password,
                None => panic!("password must be provided"),
            };
            store_key(password);
        }
        Commands::Submit(args) => {
            let password = match &args.password {
                Some(password) => password,
                None => panic!("password must be provided"),
            };
            let href = match &args.href {
                Some(href) => href,
                None => panic!("href must be provided"),
            };
            let title = match &args.title {
                Some(title) => title,
                None => panic!("title must be provided"),
            };
            let ledger = false;
            let message = create_message(password, href, title, ledger, None).await;
            send(message).await;
        }
        Commands::Vote(args) => {
            let password = match &args.password {
                Some(password) => password,
                None => panic!("password must be provided"),
            };
            let href = match &args.href {
                Some(href) => href,
                None => panic!("href must be provided"),
            };
            let ledger = false;
            let title = String::new(); // Empty title
            let message = create_message(password, href, &title, ledger, None).await;
            send(message).await;
        }
        Commands::SubmitLedger(args) => {
            let href = match &args.href {
                Some(href) => href,
                None => panic!("href must be provided"),
            };
            let title = match &args.title {
                Some(title) => title,
                None => panic!("title must be provided"),
            };
            let ledger = true;
            let password = String::new();
            let address_index = args.address_index;
            let message = create_message(&password, href, title, ledger, address_index).await;
            send(message).await;
        }
        Commands::VoteLedger(args) => {
            let href = match &args.href {
                Some(href) => href,
                None => panic!("href must be provided"),
            };
            let ledger = true;
            let password = String::new();
            let title = String::new(); // Empty title
            let address_index = args.address_index;
            let message = create_message(&password, href, &title, ledger, address_index).await;
            send(message).await;

        }
    }
}

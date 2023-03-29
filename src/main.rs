// @format
use clap::{Args, Parser, Subcommand};
use dirs::home_dir;
use ethers::{
    contract::{EthAbiType, Eip712},
    core::{
        types::{transaction::eip712::Eip712, U256, Signature},
    },
    signers::{LocalWallet, Signer},
};
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};
use serde_json::json;

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
        Err(error) => panic!("Problem reading and/or decrypting the key store")
    };
    return wallet;
}

#[derive(Debug, Clone, Eip712, EthAbiType)]
#[eip712(
    name = "replica",
    version = "1",
    chain_id = 666,
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

async fn create_message(password: &String, href: &String, title: &String) -> String {
    let wallet = read_key(password);
    let message = Message {
        title: String::from(title),
        href: String::from(href),
        r#type: String::from("amplify"),
        timestamp: U256::from(get_unix_time())
    };
    let sig = wallet.sign_typed_data(&message).await.expect("failed to sign typed data");
    let body = json!({
        "title": message.title,
        "href": message.href,
        "type": message.r#type,
        "timestamp": message.timestamp,
        "signature": sig.to_string(),
    });
    return body.to_string();
}

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
        },
        Commands::Submit(args) => {
            let password = match &args.password {
                Some(password) => password,
                None => panic!("password must be provided"),
            };
            let href = match &args.href{
                Some(href) => href,
                None => panic!("href must be provided"),
            };
            let title = match &args.title{
                Some(title) => title,
                None => panic!("title must be provided"),
            };
            let message = create_message(password, href, title).await;
            dbg!(message);
        }
    }
}

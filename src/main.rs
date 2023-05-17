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
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};
use std::{fs, io::Write};

// Define the CLI parser and its options
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

// An enumeration representing the various subcommands supported by the CLI.
#[derive(Subcommand)]
enum Commands {
    /// Submits an article.
    Submit(SubmitArgs),
    /// Votes for an article.
    Vote(VoteArgs),
    /// Configurates the default values.
    Config(ConfigArgs),
}

/// `SubmitArgs` constructs the message arguments for submitting.
#[derive(Args)]
struct SubmitArgs {
    href: Option<String>,
    title: Option<String>,
    password: Option<String>,
}

/// `VoteArgs` constructs the message arguments for voting.
#[derive(Args)]
struct VoteArgs {
    href: Option<String>,
    password: Option<String>,
}

/// `ConfigArgs` contains the configurable options.
#[derive(Args)]
struct ConfigArgs {
    /// Shows the config file
    #[arg(short = 's', long = "show")]
    show: bool,
    /// Sets ledger as the default wallet, if false keysotre file is used
    #[arg(short = 'l', long = "ledger")]
    ledger: Option<bool>,
    /// Sets the address index for the ledger
    #[arg(short = 'i', long = "index")]
    ledger_address_index: Option<usize>,
    /// Sets the node endpoint
    #[arg(short = 'e', long = "endpoint")]
    endpoint: Option<String>,
    /// Sets the path to the keysore file
    #[arg(short = 'k', long = "keystore")]
    keystore: Option<String>,
    /// Resets config to default
    #[arg(short = 'r', long = "reset")]
    reset: bool,
}

// Config structs default values
#[derive(Debug, Serialize, Deserialize)]
struct Config {
    endpoint: String,
    use_ledger: bool,
    ledger_address_index: usize,
    path_to_keystore: String,
}

// Default Config
impl Default for Config {
    fn default() -> Self {
        Config {
            use_ledger: true,
            ledger_address_index: 0,
            endpoint: "https://news.kiwistand.com/api/v1/messages".to_string(),
            path_to_keystore: "<Path>".to_string(),
        }
    }
}

// Define the EIP-712 message struct
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

// Returns the path to the user data directory ".kiwistan".
fn get_dir_path() -> PathBuf {
    let mut config_dir = home_dir().unwrap();
    config_dir.push(".kiwistand");
    config_dir
}

// Returns the path to the configuraion file.
// The configuration file is located in the user's home directory in the ".kiwistand" folder.
fn get_config_path() -> PathBuf {
    let mut config_dir = home_dir().unwrap();
    config_dir.push(".kiwistand");
    config_dir.push("config.toml");
    config_dir
}

// Reads the config file
fn read_config() -> Config {
    let config_path = get_config_path();
    if config_path.exists() {
        let config_str = fs::read_to_string(config_path).expect("Error: Couldn't read config file");
        toml::from_str(&config_str).expect("Error: Couldn't parse config file")
    } else {
        let config = Config::default();
        write_config(&config);
        config
    }
}

// Writes to the config file
fn write_config(config: &Config) {
    let config_path = get_config_path();
    if let Some(parent) = config_path.parent() {
        fs::create_dir_all(parent).expect("Error: Couldn't create config directory");
    }
    let mut file = fs::File::create(config_path).expect("Error: Couldn't create config file");
    let config_str = toml::to_string_pretty(config).expect("Error: Couldn't serialize config");
    file.write_all(config_str.as_bytes())
        .expect("Error: Couldn't write config file");
}

// Overwrites the existing config file with the default values.
fn overwrite_config() {
    let config = Config::default();
    let config_str = toml::to_string(&config).unwrap();
    fs::write(get_config_path(), config_str).unwrap();
}

// Reads the key store and returns a `LocalWallet` instance.
//
// The key store is decrypted using the given password.
// If there is an issue with reading or decrypting the key store, the function will panic.
fn read_key(password: &String) -> LocalWallet {
    let mut key_path = get_dir_path();
    key_path.push("key");

    match LocalWallet::decrypt_keystore(key_path, password) {
        Ok(wallet) => wallet,
        Err(_error) => panic!("Problem reading and/or decrypting the key store"),
    }
}

// Returns the current Unix time in seconds.
fn get_unix_time() -> u64 {
    let start = SystemTime::now();
    let now = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    now.as_secs()
}

// Signs the given EIP-712 message with a Ledger device.
//
// The Ledger device is accessed using the provided address index.
// Returns the signature generated by the Ledger device.
async fn sign_ledger(message: &Message, ledger_address_index: usize) -> Signature {
    let ledger = Ledger::new(HDPath::LedgerLive(ledger_address_index), 1u64)
        .await
        .unwrap();

    ledger
        .sign_typed_struct(message)
        .await
        .expect("failed to sign typed data")
}

// Signs the given EIP-712 message with a `LocalWallet` instance.
//
// Returns the generated signature.
async fn sign(wallet: LocalWallet, message: &Message) -> Signature {
    wallet
        .sign_typed_data(message)
        .await
        .expect("Couldn't sign message")
}

// Creates a signed EIP-712 message using the provided.
//
// Returns a `Value` instance containing the signed message.
async fn create_message(
    password: Option<String>,
    href: &String,
    title: &String,
    ledger: bool,
) -> Value {
    let timestamp = get_unix_time();
    let message = Message {
        title: String::from(title),
        href: String::from(href),
        r#type: String::from("amplify"),
        timestamp: U256::from(timestamp),
    };
    let config = read_config();
    let sig = if ledger {
        let index = config.ledger_address_index;
        sign_ledger(&message, index).await
    } else {
        let pw = match &password {
            Some(password) => password,
            None => panic!("password must be provided"),
        };
        let wallet = read_key(pw);
        sign(wallet, &message).await
    };
    // TODO: We should actually test this signature against the signature
    // from JS and make sure they're equal.
    let body = json!({
        "title": message.title,
        "href": message.href,
        "type": message.r#type,
        "timestamp": timestamp,
        "signature": format!("0x{}", sig),
    });

    body
}

// Sends the signed EIP-712 message to the Kiwistand server.
//
// The message is sent as a JSON payload in an HTTP POST request.
// If the request fails, the function will panic.
async fn send(message: Value) {
    let client = reqwest::Client::new();
    let config = read_config();
    dbg!(&message);
    let result = client.post(&config.endpoint).json(&message).send().await;

    let response = match result {
        Ok(response) => response,
        Err(_error) => panic!("Failed sending message"),
    };
    let body = response.text().await;
    if let Err(e) = dbg!(body) {
        eprintln!("Error: {:?}", e);
    }
}

// The entry point of the application.
//
// Parses command-line arguments and calls the appropriate subcommand functions based on user input.
#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match &cli.command {
        // Interacts with the config file
        Commands::Config(args) => {
            if args.reset {
                overwrite_config();
                println!("Configuration reset to default")
            }
            // initialiaze variable `config` after `reset`, so broken config file can be overwritten.
            let mut config = read_config();
            if args.show {
                println!("Current Configuration:");
                println!("  Ledger: {}", config.use_ledger);
                println!("  Ledger Index: {}", config.ledger_address_index);
                println!("  Endpoint: {}", config.endpoint);
                println!("  Keystore: {}", config.path_to_keystore)
            }
            if let Some(ledger) = args.ledger {
                config.use_ledger = ledger;
                write_config(&config);
                println!("Configuration updated -> Ledger: {}", config.use_ledger);
            }
            if let Some(ledger_index) = args.ledger_address_index {
                config.ledger_address_index = ledger_index;
                write_config(&config);
                println!(
                    "Configuration updated -> Ledger Index: {}",
                    config.ledger_address_index
                );
            }
            if let Some(endpoint) = &args.endpoint {
                config.endpoint = endpoint.clone();
                write_config(&config);
                println!("Configuration updated -> Endpoint: {}", config.endpoint);
            }
            if let Some(path) = &args.keystore {
                config.path_to_keystore = path.to_string();
                write_config(&config);
                println!(
                    "Configuration updated -> Keystore: {}",
                    config.path_to_keystore
                )
            }
        }

        // Submit a link with the given href and title
        Commands::Submit(args) => {
            let config = read_config();

            let ledger = config.use_ledger;
            let href = match &args.href {
                Some(href) => href,
                None => panic!("href must be provided"),
            };
            let title = match &args.title {
                Some(title) => title,
                None => panic!("title must be provided"),
            };

            // Depending if using ledger or keystore, changes pass down values
            if ledger {
                let message = create_message(None, href, title, ledger).await;
                send(message).await;
            } else {
                let password = args.password.clone();
                let message = create_message(password, href, title, ledger).await;
                send(message).await;
            }
        }

        // Vote for a link with the given href.
        Commands::Vote(args) => {
            let config = read_config();

            let ledger = config.use_ledger;
            let href = match &args.href {
                Some(href) => href,
                None => panic!("href must be provided"),
            };
            let title = String::new(); // Empty string as title

            // Depending if using ledger or keystore, changes pass down values
            if ledger {
                let message = create_message(None, href, &title, ledger).await;
                send(message).await;
            } else {
                let password = args.password.clone();
                let message = create_message(password, href, &title, ledger).await;
                send(message).await;
            }
        }
    }
}

// Unit test
#[cfg(test)]
mod tests {
    use super::*;
    use ethers::{core::types::H160, signers::Wallet};

    // Test for comparing signatures
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

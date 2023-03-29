// @format
use clap::{Args, Parser, Subcommand};
use dirs::home_dir;
use ethers::signers::LocalWallet;
use std::fs;
use std::path::Path;

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
}

#[derive(Args)]
struct InitArgs {
    password: Option<String>,
}

fn store_key(password: &String) {
    let mut config_dir = home_dir().unwrap();
    config_dir.push(".kiwistand");
    let _ = fs::create_dir(&config_dir);
    let name = "key";

    let mut key_path = config_dir.to_path_buf();
    key_path.push(name);
    if !Path::new(&key_path).exists() {
        let mut rng = rand::thread_rng();
        LocalWallet::new_keystore(&config_dir, &mut rng, password, Some(name)).unwrap();
        return;
    }
    println!("Bailed from creating key store as it already exists");
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Init(args) => {
            let password = match &args.password {
                Some(password) => password,
                None => panic!("password must be provided"),
            };
            dbg!(password);
            store_key(password);
        }
    }
}

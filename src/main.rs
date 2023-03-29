// @format
use clap::{Args, Parser, Subcommand};
use dirs::home_dir;
use ethers::{signers::LocalWallet};
use std::fs;

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

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Init(args) => {
            let mut config_dir = home_dir().unwrap();
            config_dir.push(".kiwistand");
            let _ = fs::create_dir(&config_dir);

            let password = match &args.password {
                Some(password) => password,
                None => panic!("password must be provided")

            };
            let mut rng = rand::thread_rng();
            let name = Some("key");
            let key = 
                LocalWallet::new_keystore(&config_dir, &mut rng, password, name).unwrap();
        }
    }
}

//#[tokio::main]
//async fn main() -> Result<(), Box<dyn std::error::Error>> {
//    let provider = Provider::<Http>::try_from("https://")?;
//    let block_number: U64 = provider.get_block_number().await?;
//    println!("{block_number}");
//
//    Ok(())
//}

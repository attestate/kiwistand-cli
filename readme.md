# kiwistand-cli

Kiwistand is a Rust command-line application for submitting links with their
titles to the Kiwinews service hosted at news.kiwistand.com. The application
signs the submitted messages using Ethereum's EIP-712 typed data.
<br> 


## Prerequisites

- Rust and Cargo installed, for help see [guide](https://doc.rust-lang.org/book/ch01-01-installation.html")
- Wallet with an [kiwistand NFT](#nft-required)
- An internet connection
- Recommended to use a Ledger Wallet
<br> 


## Set Up

1. Clone this repository:
```console
git clone https://github.com/attestate/kiwistand-cli.git
```

2. Navigate to the cloned directory:
```console
cd kiwistand-cli
```

3. Initiate the config file and set it up:
```console
cargo run config
```

4. Execute commands in the directory:
```console
cargo run [subcommands]
```

*we use `cargo` to execute the commands for now*

## Config

You can configure how to use kiwistand-cli by setting the parameters as you wish.

To show your current parameters:
```console
cargo run config -s
```

If you want to see the options available to you:
```console
cargo run help config
```

### Non Ledger Wallet
If you don't use a Ledger wallet:
- set Ledger to `false` 
- set the path to your keysotre file

*don't forget to use your password when submitting or voting.*

## Subcommands
When using a Ledger wallet, make sure to have it connected and unlocked.

When using a keystore file, append the password.

### Submit a Link

To submit a link with its title, run the following command:

```console
cargo run submit [Link] [Title]
```

- `[Link]` *insert your own link*
- `[Title]` *insert your own title as a string (in "double quotation marks")*


### Vote for a Link

To vote for a post, you have to resubmit the link.
run the following command:

```console
cargo run vote [Link]
```

- `[Link]` *insert the link you want to vote for*

<br> 

## NFT Required!

Consider that your address must be a "minter" of the [kiwistand NFT](https://kiwistand.com), otherwise your submission won't be accepted by the nodes.
This means, it must be the `to` address in the token's first `Transfer(from, to, tokenId)` event where `from=address(0)`. Otherwise
your message will be dropped and considered invalid.

<br> 

## License

This project is licensed under the MIT License.

## Live Service

Kiwinews is live at [news.kiwistand.com](https://news.kiwistand.com).


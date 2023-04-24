# kiwistand-cli

Kiwistand is a Rust command-line application for submitting links with their
titles to the Kiwinews service hosted at news.kiwistand.com. The application
signs the submitted messages using Ethereum's EIP-712 typed data.
<br> 


## Prerequisites

- Rust and Cargo installed, for help see [guide](https://doc.rust-lang.org/book/ch01-01-installation.html")
- Wallet with an [kiwistand NFT](#nft-required)
- An internet connection
<br> 


## Quick Installation Tutorial

1. Clone this repository:
```console
git clone https://github.com/attestate/kiwistand-cli.git
```

2. Navigate to the cloned directory:
```console
cd kiwistand-cli
```

3. Execute commands in the directory:
```console
cargo [options]
```
*we use `cargo` to execute the commands for now*


### Submit a Link with a Ledger

**Make sure to have Ledger Live open and your Ledger connected**

To submit a link with its title, run the following command:

```console
cargo run -- submit-ledger [Link] [Title] [WalletIndex]
```

- `[Link]` *insert your own link*
- `[Title]` *insert your own title as a string (in "double quotation marks")*
- `[WalletIndex]` *insert an index starting at 0 to choose the wallet, if let empty reverts to standart (0)


### Vote for a Link with a Ledger

**Make sure to have Ledger Live open and your Ledger connected**

To vote for a post, you have to resubmit the link.
run the following command:

```console
cargo run -- vote-ledger [Link] [WalletIndex]
```

- `[Link]` *insert your own link*
- `[WalletIndex]` *insert an index starting at 0 to choose the wallet, if let empty reverts to standart (0)


### Submit a Link

To submit a link with its title, run the following command:

```console
cargo run -- submit [Password] [Link] [Title]
```

- `[Password]` *insert your password*
- `[Link]` *insert your own link*
- `[Title]` *insert your own title as a string (in "double quotation marks")*


### Vote for a Link

To vote for a post, you have to resubmit the link.
run the following command:

```console
cargo run -- vote [Password] [Link]
```

- `[Password]` *insert your password*
- `[Link]` *insert your own link*


### Generating a New Keystore File

A keystore file is an encrypted container that stores a private key, allowing
you to securely sign messages. To generate a new keystore file, run the
following command:

```console
cargo run -- init <your_password>
```

Replace "your_password" with your desired password. The keystore file will be
generated and stored in the .kiwistand directory under your home directory. The
keystore file is at `$home/.kiwistand/key`.

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


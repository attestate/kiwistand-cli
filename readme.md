# kiwistand-cli

Kiwistand is a Rust command-line application for submitting links with their
titles to the Kiwinews service hosted at news.kiwistand.com. The application
signs the submitted messages using Ethereum's EIP-712 typed data.

## Prerequisites

- Rust and Cargo installed
- An internet connection

## Quick Installation Tutorial

1. Clone this repository:

   git clone https://github.com/attestate/kiwistand-cli.git

2. Navigate to the cloned directory:

   cd kiwistand

3. Make sure you adjust the target endpoint in `src/main.rs` from
   `http://localhost` to the node you want to submit the link to.

## Submitting a Link with a Ledger

- Make sure to have Ledger Live open and your Ledger connected

To submit a link with its title, run the following command:

```
cargo run -- submit-ledger https://warpcast.com/timdaub/0x1f54f8 "Kiwistand is live"
```

Replace "https://warpcast.com", and "Kiwistand is live" with your desired link,
and title, respectively.

## Submitting a Link

To submit a link with its title, run the following command:

```
cargo run -- submit abc https://warpcast.com/timdaub/0x1f54f8 "Kiwistand is live"
```

Replace the "abc" password, "https://warpcast.com", and "Kiwistand is live"
with your desired link, and title, respectively.

## Generating a New Keystore File

A keystore file is an encrypted container that stores a private key, allowing
you to securely sign messages. To generate a new keystore file, run the
following command:

```
cargo run -- init <your_password>
```

Replace "your_password" with your desired password. The keystore file will be
generated and stored in the .kiwistand directory under your home directory. The
keystore file is at `$home/.kiwistand/key`.

## NFT Required!

Consider that your address must be a "minter" of the
http://kiwistand.com NFT, otherwise your submission won't be accepted by the
nodes. This means, it must be the `to` address in the token's first
`Transfer(from, to, tokenId)` event where `from=address(0)`. Otherwise
your message will be dropped and considered invalid.

## License

This project is licensed under the MIT License.

## Live Service

Kiwinews is live at [news.kiwistand.com](https://news.kiwistand.com).


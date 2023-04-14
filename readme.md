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

## Submitting a Link

To submit a link with its title, run the following command:

```
cargo run -- submit abc https://warpcast.com/timdaub/0x1f54f8 "Kiwistand is live"
```

Replace "your_password", "https://example.com", and "Example Title" with your
desired password, link, and title, respectively.

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

However, consider that your address must be a minter of the
http://kiwistand.com NFT, otherwise your submission won't be accepted by the
nodes.

## License

This project is licensed under the MIT License.

## Live Service

Kiwinews is live at [news.kiwistand.com](https://news.kiwistand.com).


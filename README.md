# Wallet Inspector CLI

A CLI-based Solana wallet inspector built with Rust using the Solana SDK and Clap.
---

# Features

- Wallet creation and loading
- Balance inspection
- Devnet SOL airdrops
- SOL transfers
- Async RPC communication using Tokio
- Command-line interface using Clap

---

# Tech Stack

- Rust
- Tokio
- Solana SDK
- Solana Client
- Clap


---

# Running the Project

## Build the project

```bash
cargo build
```

---

## Run the project

```bash
cargo run
```

---

# Available Commands

---

# 1. Create Wallet

Creates or loads a local Solana wallet.

```bash
cargo run -- create-wallet
```

Example output:

```txt
Address: 9xQeWvG816bUx9EP...
```

---

# 2. Inspect Wallet Balance

Checks the balance of a wallet address on Solana Devnet.

```bash
cargo run -- inspect -a <WALLET_ADDRESS>
```

Example:

```bash
cargo run -- inspect -a 9xQeWvG816bUx9EP...
```

---

# 3. Airdrop SOL on Devnet

Airdrops SOL to a wallet address.

```bash
cargo run -- inspect -a <WALLET_ADDRESS> --drop <AMOUNT>
```

Example:

```bash
cargo run -- inspect -a 9xQeWvG816bUx9EP... --drop 1
```

This command:

- Requests an airdrop on Devnet
- Confirms the transaction
- Displays updated balance

---

# 4. Send SOL

Transfers SOL from your local wallet to another wallet.

```bash
cargo run -- send_sol <RECEIVER_ADDRESS> <AMOUNT>
```

Example:

```bash
cargo run -- send_sol 9xQeWvG816bUx9EP... 0.1
```

---

# Solana Devnet

This project uses:

```txt
https://api.devnet.solana.com
```
---

# Important Notes

- The wallet must contain Devnet SOL before sending transactions
- Invalid wallet addresses will cause parsing errors
- Transactions are executed asynchronously using Tokio
- This project is intended for learning and development purposes

---

# Run Help Command

```bash
cargo run -- --help
```

Or for a specific command:

```bash
cargo run -- inspect --help
```

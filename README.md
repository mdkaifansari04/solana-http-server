# 🔗 Solana Rust HTTP API

A simple Rust API that interacts with the Solana blockchain (Devnet) using the Axum framework.

## ✨ Features

- ✅ Airdrop SOL to any wallet
- ✅ Get wallet balance
- ⚡ Built with `axum` (Rust web framework)
- ⚡ Uses Solana Devnet (test network)

## 📦 Tech Stack

- [Rust](https://www.rust-lang.org/)
- [Axum](https://docs.rs/axum/)
- [Solana SDK](https://docs.rs/solana-sdk)
- [Solana Client](https://docs.rs/solana-client)

## 📁 Project Structure

```

src/
├── main.rs        # Axum server and route setup
├── route.rs       # Balance + Airdrop route handlers

```

## 🚀 Run Locally

### 1. Install dependencies

```bash
cargo build
```

### 2. Run the local server

```bash
cargo run
```

Server will run at: `http://localhost:3000`

### 3. Test Endpoints

**Airdrop**

```bash
curl -X POST http://localhost:3000/airdrop \
  -H "Content-Type: application/json" \
  -d '{"address": "<WALLET_ADDRESS>"}'
```

**Get Balance**

```bash
curl http://localhost:3000/balance/<WALLET_ADDRESS>
```

## 🌐 Deploy

### 1. [Render.com](https://render.com)

- Build command: `cargo build --release`
- Start command: `cargo run --release`
- Port: `3000`

## 📘 Notes

- Make sure to use Devnet wallet keys.
- RPC used: `https://api.devnet.solana.com`

## 🧠 Author

Made with 💻 by [@mdkaifansari04](https://github.com/mdkaifansari04)

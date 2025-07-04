# TON-SOLANA SMART WALLET

Proof of concept for TON-Solana smart wallet integration with cross-chain wallet creation and DEX trading capabilities.

## Overview

This project demonstrates a cross-chain integration between TON and Solana blockchains, allowing users to:
- Connect their TON wallet
- Create Solana smart wallets through TON wallet signatures
- Execute trades on Solana DEXs using the created smart wallets

## Project Structure

```
├── frontend/              # Web interface for TON wallet connection
├── solana-contracts/      # Solana smart contracts (factory + wallet)
├── ton-integration/       # TON wallet integration logic
└── docs/                  # Documentation
```

## Features

- **TON Wallet Connection**: Connect and authenticate with TON wallets
- **Cross-Chain Wallet Creation**: Generate Solana smart wallets from TON signatures
- **Smart Contract Factory**: Deploy and manage Solana smart wallet contracts
- **DEX Integration**: Trade on Solana decentralized exchanges

## Development Status

🚧 **This is a proof of concept project under active development**

## Tech Stack

- **Frontend**: React/Next.js (deployable to Vercel)
- **Solana Contracts**: Anchor/Rust
- **TON Integration**: TON SDK
- **Blockchain**: Solana (devnet) + TON (testnet)
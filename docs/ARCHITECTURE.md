# TON-Solana Smart Wallet Architecture

## Overview

This project implements a cross-chain smart wallet system that allows TON users to control Solana accounts using TON wallet signatures, bypassing TonConnect limitations in Telegram mini-apps.

## Core Components

### 1. Factory Contract (`initialize_factory`, `create_wallet`)

The Factory contract manages the creation and registry of smart wallets.

**Functions:**
- `initialize_factory()` - Sets up the factory with authority
- `create_wallet(ton_public_key, nonce)` - Creates a new smart wallet for a TON user

**Storage:**
- Authority address
- Wallet counter
- Mapping of TON pubkeys to Solana addresses

### 2. Smart Wallet Contract (`execute_transaction`, `set_wallet_status`)

Each user gets their own smart wallet contract that:
- Stores the user's TON public key
- Verifies TON signatures for all operations
- Executes authorized transactions

**Functions:**
- `execute_transaction(message, signature, instruction_data)` - Executes operations with TON signature
- `set_wallet_status(is_active, message, signature)` - Pause/unpause wallet

**Storage:**
- TON public key (32 bytes)
- Factory reference
- Owner address
- Nonce
- Active status

### 3. Signature Verification

The core innovation is verifying TON ED25519 signatures within Solana contracts:

```rust
fn verify_ton_signature(
    ton_public_key: &[u8; 32],
    message: &[u8],
    signature: &[u8; 64],
) -> Result<()>
```

This function:
1. Takes the TON public key stored in the wallet
2. Hashes the signed message with SHA-256 (TON standard)
3. Verifies the ED25519 signature
4. Returns success/failure

## User Flow

### Setup Phase
1. **Deploy Wallet**: User calls `create_wallet()` with their TON public key
2. **Fund Wallet**: User deposits USDT via cross-chain bridge (Stargate/Omnichain)

### Operation Phase
1. **Sign Message**: User signs transaction data with TON wallet using `signData()`
2. **Submit Transaction**: Frontend submits signed message to Solana contract
3. **Verify & Execute**: Contract verifies TON signature and executes operation
4. **Result**: Transaction completed on Solana

### Supported Operations
- **Transfer**: Send tokens to another address
- **Swap**: Trade tokens on DEX (Jupiter/Raydium integration)
- **Withdraw**: Remove funds back to TON via bridge

## Security Model

### Access Control
- Only transactions signed by the correct TON private key are accepted
- Each wallet is isolated and can only be controlled by its TON key owner
- Factory authority can pause the system if needed

### Signature Verification
- Uses standard ED25519 cryptography (same as TON)
- Message hashing with SHA-256 (TON standard)
- Replay protection via nonces in signed messages

### Smart Contract Security
- PDA (Program Derived Address) for deterministic wallet addresses
- Anchor framework for secure account validation
- Token account ownership verification

## Technical Implementation

### Message Format
Signed messages contain:
```json
{
  "wallet_address": "Solana address",
  "operation": "transfer|swap|withdraw",
  "amount": "token amount",
  "recipient": "destination address",
  "timestamp": "unix timestamp",
  "nonce": "replay protection"
}
```

### Deployment
- **Devnet**: For testing and development
- **Mainnet**: Production deployment
- **Program ID**: `Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS`

## Benefits

1. **No TonConnect Required**: Works in Telegram mini-apps without TonConnect
2. **Native TON UX**: Users sign with familiar TON wallet interface
3. **Solana Access**: Full access to Solana DeFi ecosystem
4. **Cross-Chain**: Seamless USDT movement between TON and Solana
5. **Security**: Cryptographic proof of authorization

## Future Enhancements

1. **DEX Integration**: Jupiter aggregator for optimal swap routing
2. **Multi-Token Support**: Beyond USDT to other cross-chain assets
3. **Batch Operations**: Multiple transactions in single signature
4. **Social Recovery**: Multi-sig with backup TON keys
5. **Gas Optimization**: Compressed transactions and batching
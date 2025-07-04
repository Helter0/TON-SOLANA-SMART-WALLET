use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};
use ed25519_dalek::{PublicKey, Signature, Verifier};
use sha2::{Digest, Sha256};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod ton_solana_wallet {
    use super::*;

    /// Initialize the factory
    pub fn initialize_factory(ctx: Context<InitializeFactory>) -> Result<()> {
        let factory = &mut ctx.accounts.factory;
        factory.authority = ctx.accounts.authority.key();
        factory.wallet_count = 0;
        Ok(())
    }

    /// Create a new smart wallet for a TON user
    pub fn create_wallet(
        ctx: Context<CreateWallet>,
        ton_public_key: [u8; 32],
        nonce: u64,
    ) -> Result<()> {
        let factory = &mut ctx.accounts.factory;
        let wallet = &mut ctx.accounts.wallet;

        wallet.factory = factory.key();
        wallet.ton_public_key = ton_public_key;
        wallet.owner = ctx.accounts.owner.key();
        wallet.nonce = nonce;
        wallet.is_active = true;

        factory.wallet_count += 1;

        emit!(WalletCreated {
            wallet_address: wallet.key(),
            ton_public_key,
            owner: ctx.accounts.owner.key(),
        });

        Ok(())
    }

    /// Execute a transaction with TON signature verification
    pub fn execute_transaction(
        ctx: Context<ExecuteTransaction>,
        message: Vec<u8>,
        signature: [u8; 64],
        instruction_data: Vec<u8>,
    ) -> Result<()> {
        let wallet = &ctx.accounts.wallet;

        // Verify TON signature
        verify_ton_signature(&wallet.ton_public_key, &message, &signature)?;

        // Parse and validate the message
        let tx_data: TransactionData = borsh::BorshDeserialize::deserialize(&mut instruction_data.as_slice())?;

        match tx_data.tx_type {
            TransactionType::Transfer => {
                execute_transfer(ctx, tx_data)?;
            }
            TransactionType::Swap => {
                execute_swap(ctx, tx_data)?;
            }
            TransactionType::Withdraw => {
                execute_withdraw(ctx, tx_data)?;
            }
        }

        emit!(TransactionExecuted {
            wallet: wallet.key(),
            tx_type: tx_data.tx_type,
            amount: tx_data.amount,
        });

        Ok(())
    }

    /// Pause/unpause wallet
    pub fn set_wallet_status(
        ctx: Context<SetWalletStatus>,
        is_active: bool,
        message: Vec<u8>,
        signature: [u8; 64],
    ) -> Result<()> {
        let wallet = &mut ctx.accounts.wallet;

        // Verify TON signature
        verify_ton_signature(&wallet.ton_public_key, &message, &signature)?;

        wallet.is_active = is_active;

        Ok(())
    }
}

// Helper function to verify TON signature
fn verify_ton_signature(
    ton_public_key: &[u8; 32],
    message: &[u8],
    signature: &[u8; 64],
) -> Result<()> {
    let public_key = PublicKey::from_bytes(ton_public_key)
        .map_err(|_| ErrorCode::InvalidPublicKey)?;
    
    let signature = Signature::from_bytes(signature)
        .map_err(|_| ErrorCode::InvalidSignature)?;

    // Hash the message (TON uses SHA-256 for signing)
    let mut hasher = Sha256::new();
    hasher.update(message);
    let message_hash = hasher.finalize();

    public_key
        .verify(&message_hash, &signature)
        .map_err(|_| ErrorCode::SignatureVerificationFailed)?;

    Ok(())
}

// Execute transfer transaction
fn execute_transfer(ctx: Context<ExecuteTransaction>, tx_data: TransactionData) -> Result<()> {
    require!(ctx.accounts.wallet.is_active, ErrorCode::WalletInactive);

    let transfer_ctx = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        Transfer {
            from: ctx.accounts.from_token_account.to_account_info(),
            to: ctx.accounts.to_token_account.to_account_info(),
            authority: ctx.accounts.wallet.to_account_info(),
        },
    );

    token::transfer(transfer_ctx, tx_data.amount)
}

// Execute swap transaction (placeholder)
fn execute_swap(_ctx: Context<ExecuteTransaction>, _tx_data: TransactionData) -> Result<()> {
    // TODO: Implement DEX integration (Jupiter, Raydium)
    Ok(())
}

// Execute withdraw transaction
fn execute_withdraw(ctx: Context<ExecuteTransaction>, tx_data: TransactionData) -> Result<()> {
    require!(ctx.accounts.wallet.is_active, ErrorCode::WalletInactive);

    let transfer_ctx = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        Transfer {
            from: ctx.accounts.from_token_account.to_account_info(),
            to: ctx.accounts.to_token_account.to_account_info(),
            authority: ctx.accounts.wallet.to_account_info(),
        },
    );

    token::transfer(transfer_ctx, tx_data.amount)
}

// Account structures
#[derive(Accounts)]
pub struct InitializeFactory<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + std::mem::size_of::<Factory>()
    )]
    pub factory: Account<'info, Factory>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(ton_public_key: [u8; 32], nonce: u64)]
pub struct CreateWallet<'info> {
    #[account(mut)]
    pub factory: Account<'info, Factory>,
    #[account(
        init,
        payer = owner,
        space = 8 + std::mem::size_of::<SmartWallet>(),
        seeds = [b"wallet", ton_public_key.as_ref(), nonce.to_le_bytes().as_ref()],
        bump
    )]
    pub wallet: Account<'info, SmartWallet>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ExecuteTransaction<'info> {
    pub wallet: Account<'info, SmartWallet>,
    #[account(mut)]
    pub from_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub to_token_account: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct SetWalletStatus<'info> {
    #[account(mut)]
    pub wallet: Account<'info, SmartWallet>,
}

// Data structures
#[account]
pub struct Factory {
    pub authority: Pubkey,
    pub wallet_count: u64,
}

#[account]
pub struct SmartWallet {
    pub factory: Pubkey,
    pub ton_public_key: [u8; 32],
    pub owner: Pubkey,
    pub nonce: u64,
    pub is_active: bool,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy)]
pub struct TransactionData {
    pub tx_type: TransactionType,
    pub amount: u64,
    pub recipient: Option<Pubkey>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy)]
pub enum TransactionType {
    Transfer,
    Swap,
    Withdraw,
}

// Events
#[event]
pub struct WalletCreated {
    pub wallet_address: Pubkey,
    pub ton_public_key: [u8; 32],
    pub owner: Pubkey,
}

#[event]
pub struct TransactionExecuted {
    pub wallet: Pubkey,
    pub tx_type: TransactionType,
    pub amount: u64,
}

// Error codes
#[error_code]
pub enum ErrorCode {
    #[msg("Invalid public key")]
    InvalidPublicKey,
    #[msg("Invalid signature")]
    InvalidSignature,
    #[msg("Signature verification failed")]
    SignatureVerificationFailed,
    #[msg("Wallet is inactive")]
    WalletInactive,
}
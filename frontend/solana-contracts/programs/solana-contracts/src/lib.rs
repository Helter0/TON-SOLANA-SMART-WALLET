use anchor_lang::prelude::*;

declare_id!("CEzs3bEbqXFZ528gTj3RXDctityB7PZj5VmsEesz5oQ7");

#[program]
pub mod solana_contracts {
    use super::*;

    /// Initialize the factory
    pub fn initialize_factory(ctx: Context<InitializeFactory>) -> Result<()> {
        let factory = &mut ctx.accounts.factory;
        factory.authority = ctx.accounts.authority.key();
        factory.wallet_count = 0;
        msg!("Factory initialized with authority: {}", factory.authority);
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

        msg!("Wallet created: {}", wallet.key());
        Ok(())
    }

    /// Execute a transaction with TON signature verification (simplified for POC)
    pub fn execute_transaction(
        ctx: Context<ExecuteTransaction>,
        message: Vec<u8>,
        signature: [u8; 64],
        amount: u64,
    ) -> Result<()> {
        let wallet = &ctx.accounts.wallet;

        // For POC: Basic verification
        require!(wallet.is_active, ErrorCode::WalletInactive);
        require!(message.len() > 0, ErrorCode::InvalidMessage);
        require!(signature.len() == 64, ErrorCode::InvalidSignature);

        msg!("Executing transaction for wallet: {}", wallet.key());
        msg!("Amount: {}", amount);
        msg!("Message length: {}", message.len());

        emit!(TransactionExecuted {
            wallet: wallet.key(),
            amount,
        });

        Ok(())
    }
}

// Account structures
#[derive(Accounts)]
pub struct InitializeFactory<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + 32 + 8 // discriminator + pubkey + u64
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
        space = 8 + 32 + 32 + 32 + 8 + 1, // discriminator + factory + ton_pubkey + owner + nonce + bool
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
    pub amount: u64,
}

// Error codes
#[error_code]
pub enum ErrorCode {
    #[msg("Invalid message")]
    InvalidMessage,
    #[msg("Invalid signature")]
    InvalidSignature,
    #[msg("Wallet is inactive")]
    WalletInactive,
}
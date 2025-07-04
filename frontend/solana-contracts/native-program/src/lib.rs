use solana_program::{
    account_info::AccountInfo,
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
};

entrypoint!(process_instruction);

fn process_instruction(
    program_id: &Pubkey,
    _accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!("TON-Solana Smart Wallet Program Called!");
    msg!("Program ID: {}", program_id);
    msg!("Instruction data length: {}", instruction_data.len());
    
    // Простая проверка данных
    if instruction_data.len() > 0 {
        msg!("First byte: {}", instruction_data[0]);
        
        match instruction_data[0] {
            0 => msg!("Initialize Factory"),
            1 => msg!("Create Wallet"),
            2 => msg!("Execute Transaction"),
            _ => msg!("Unknown instruction"),
        }
    }
    
    Ok(())
}
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

use std::convert::TryInto;

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct InputDataStorage {
    pub answer: u32,
}

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!("MODULE 4 - FINAL PROJECT [MING MANANGAN]");

    let accounts_iter = &mut accounts.iter();

    let account = next_account_info(accounts_iter)?;

    if account.owner != program_id {
        msg!("INVALID PROGRAM ID");
        return Err(ProgramError::IncorrectProgramId);
    }

    let (operation, remaining_data) = instruction_data.split_first().ok_or(ProgramError::InvalidInstructionData)?;
    
    let mut result: u32 = 0;
    let mut operation_str: String = String::new();
    
    let first_number: u32 = remaining_data
        .get(..4)
        .and_then(|slice| slice.try_into().ok())
        .map(u32::from_le_bytes)
        .ok_or(ProgramError::InvalidInstructionData)?;
    
    let second_number: u32 = remaining_data
        .get(4..)
        .and_then(|slice| slice.try_into().ok())
        .map(u32::from_le_bytes)
        .ok_or(ProgramError::InvalidInstructionData)?;

    match operation {
        0 => {
            msg!("ADDITION");
            operation_str = String::from("+");
            result = first_number + second_number;
        },
        1 => {
            msg!("SUBTRACTION");
            operation_str = String::from("-");
            result = first_number - second_number;
        },
        _ => {
            msg!("INVALID OPERATION");
        },
    };

    let mut data_storage_account = InputDataStorage::try_from_slice(&account.data.borrow())?;
    data_storage_account.answer = result;
    data_storage_account.serialize(&mut &mut account.data.borrow_mut()[..])?;

    msg!("{} {} {} = {}", first_number, operation_str, second_number, result);
    Ok(())
}

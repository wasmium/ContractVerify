use borsh::de::BorshDeserialize;
use common_types::{ContractInstruction, VerifyOutcome};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
    sysvar::instructions::load_instruction_at_checked,
};

mod storage;
use storage::*;

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();

    let pda_account = next_account_info(accounts_iter)?;
    let sysvar_account = next_account_info(accounts_iter)?;
    let signing_account = next_account_info(accounts_iter)?;

    if pda_account.owner != program_id {
        msg!("The account is not owned by the program");
        return Err(ProgramError::IllegalOwner);
    }

    let deser_instruction_data = ContractInstruction::try_from_slice(instruction_data)?;

    let mut processor = ProcessInstruction::unpack(&pda_account.data.as_ref().borrow()).unwrap(); //TODO
    processor
        .add_signer(signing_account.key.to_bytes())
        .add_instruction(deser_instruction_data);

    msg!("PDA: {:?}", pda_account);
    msg!("sysvar: {:?}", sysvar_account);

    if !signing_account.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    let inst_0 = load_instruction_at_checked(0, sysvar_account)?;

    let outcome = processor.process(&inst_0);

    // TODO Handle Error Messages
    match outcome {
        VerifyOutcome::InvalidMessageHash => {
            msg!("OUTCOME: {:?}", "InvalidMessageHash");
            panic!()
        }

        VerifyOutcome::InvalidNewOwner => {
            msg!("OUTCOME: {:?}", "InvalidNewOwner");
            panic!()
        }

        VerifyOutcome::InvalidSignature => {
            msg!("OUTCOME: {:?}", "InvalidSignature");
            panic!()
        }

        VerifyOutcome::AddedHash => {
            msg!("OUTCOME: {:?}", "ADDED HASH");
            processor
                .pack(&mut &mut pda_account.data.borrow_mut()[..])
                .unwrap();
        }

        VerifyOutcome::SignatureVerified => {
            msg!("OUTCOME: {:?}", "SignatureVerified");
            processor
                .pack(&mut &mut pda_account.data.borrow_mut()[..])
                .unwrap();
        }

        VerifyOutcome::ReInitialized => {
            msg!("OUTCOME: {:?}", "ReInitialized");
            processor
                .pack(&mut &mut pda_account.data.borrow_mut()[..])
                .unwrap();
        }
    }

    Ok(())
}

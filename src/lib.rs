use pinocchio::{account_info::AccountInfo, entrypoint, program_error::ProgramError, pubkey::Pubkey, ProgramResult};
entrypoint!(process_instruction);

pub mod instructions;
pub use instructions::*;

pub mod state;
pub use state::*;

pinocchio_pubkey::declare_id!("DSN3Ao1WRSLJXVDH68oAfSPbhU7qYKoFkN6rv2UfnEVZ");


fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instructions_data: &[u8],
) -> ProgramResult {
     match instructions_data.split_first() {
        Some((Loan::DISCRIMINATOR, data)) => Loan::try_from((data, accounts))?.process(),
        Some((Repay::DISCRIMINATOR, _)) => Repay::try_from(accounts)?.process(),
        Some((InitializeEmergency::DISCRIMINATOR, data)) => InitializeEmergency::try_from((data, accounts))?.process(),
        Some((Pause::DISCRIMINATOR, _)) => Pause::try_from(accounts)?.process(),
        Some((Unpause::DISCRIMINATOR, _)) => Unpause::try_from(accounts)?.process(),
        Some((SetEmergencyMode::DISCRIMINATOR, data)) => SetEmergencyMode::try_from((data, accounts))?.process(),
        _ => Err(ProgramError::InvalidInstructionData)
    }
}
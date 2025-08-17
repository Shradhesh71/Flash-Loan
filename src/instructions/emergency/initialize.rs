use pinocchio::{
    account_info::AccountInfo, 
    program_error::ProgramError, 
    pubkey::Pubkey, 
    sysvars::{clock::Clock, Sysvar}, 
    ProgramResult
};

use crate::{EmergencyMode, EmergencyState};

pub struct InitializeEmergencyAccounts<'a> {
    pub emergency_account: &'a AccountInfo,
    pub admin: &'a AccountInfo,
    pub payer: &'a AccountInfo,
}

impl<'a> TryFrom<&'a [AccountInfo]> for InitializeEmergencyAccounts<'a> {
    type Error = ProgramError;
    
    fn try_from(accounts: &'a [AccountInfo]) -> Result<Self, Self::Error> {
        let [emergency_account, admin, payer, ..] = accounts else {
            return Err(ProgramError::NotEnoughAccountKeys);
        };

        Ok(Self {
            emergency_account,
            admin,
            payer,
        })
    }
}

pub struct InitializeEmergencyInstructionData {
    pub max_loan_amount: u64,
    pub max_total_outstanding: u64,
}

impl TryFrom<&[u8]> for InitializeEmergencyInstructionData {
    type Error = ProgramError;
    
    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        if data.is_empty() {
            // Use default values if no data provided
            return Ok(Self {
                max_loan_amount: 10_000_000_000,     // 10 SOL default
                max_total_outstanding: 100_000_000_000, // 100 SOL default
            });
        }

        if data.len() < 16 {
            return Err(ProgramError::InvalidInstructionData);
        }
        
        let max_loan_amount = u64::from_le_bytes(
            data[0..8].try_into().map_err(|_| ProgramError::InvalidInstructionData)?
        );
        let max_total_outstanding = u64::from_le_bytes(
            data[8..16].try_into().map_err(|_| ProgramError::InvalidInstructionData)?
        );
        
        Ok(Self {
            max_loan_amount,
            max_total_outstanding,
        })
    }
}

pub struct InitializeEmergency<'a> {
    pub accounts: InitializeEmergencyAccounts<'a>,
    pub instruction_data: InitializeEmergencyInstructionData,
}

impl<'a> TryFrom<(&'a [u8], &'a [AccountInfo])> for InitializeEmergency<'a> {
    type Error = ProgramError;
    
    fn try_from((data, accounts): (&'a [u8], &'a [AccountInfo])) -> Result<Self, Self::Error> {
        let accounts = InitializeEmergencyAccounts::try_from(accounts)?;
        let instruction_data = InitializeEmergencyInstructionData::try_from(data)?;
        
        Ok(Self {
            accounts,
            instruction_data,
        })
    }
}

impl<'a> InitializeEmergency<'a> {
    pub const DISCRIMINATOR: &'a u8 = &10;
    
    pub fn process(&self) -> ProgramResult {
        let current_timestamp = Clock::get()?.unix_timestamp;
        
        let emergency_state = EmergencyState {
            is_paused: false,
            admin: *self.accounts.admin.key(),
            max_loan_amount: self.instruction_data.max_loan_amount,
            max_total_outstanding: self.instruction_data.max_total_outstanding,
            emergency_mode: EmergencyMode::Normal,
            last_updated: current_timestamp,
            has_pending_admin: false,
            pending_admin: Pubkey::default(),
            admin_transfer_timestamp: 0,
        };
        
        let mut emergency_data = self.accounts.emergency_account.try_borrow_mut_data()?;
        let emergency_state_bytes = unsafe {
            core::slice::from_raw_parts(
                &emergency_state as *const EmergencyState as *const u8,
                core::mem::size_of::<EmergencyState>(),
            )
        };
        emergency_data[..core::mem::size_of::<EmergencyState>()].copy_from_slice(emergency_state_bytes);
        
        Ok(())
    }
}

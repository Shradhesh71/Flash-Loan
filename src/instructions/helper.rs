use pinocchio::{program_error::ProgramError, ProgramResult};

use crate::{EmergencyMode, EmergencyState};

#[repr(C, packed)]
pub struct LoanData {
    pub protocol_token_account: [u8; 32],
    pub balance: u64,
}

pub fn get_token_account(data: &[u8]) -> u64 {
    unsafe {
        *(data.as_ptr().add(64) as *const u64)
    }
}

/// validation function to check if operation is allowed
pub fn validate_emergency_state(emergency_state: &EmergencyState, loan_amount: u64) -> ProgramResult {
    if emergency_state.is_paused {
        return Err(ProgramError::InvalidAccountData);
    }
    
    match emergency_state.emergency_mode {
        EmergencyMode::Normal => {
            if loan_amount > emergency_state.max_loan_amount {
                return Err(ProgramError::InvalidAccountData);
            }
        },
        EmergencyMode::Limited => {
            let limited_amount = emergency_state.max_loan_amount / 2;
            if loan_amount > limited_amount {
                return Err(ProgramError::InvalidAccountData);
            }
        },
        EmergencyMode::Emergency => {
            let emergency_amount = emergency_state.max_loan_amount / 4;
            if loan_amount > emergency_amount {
                return Err(ProgramError::InvalidAccountData);
            }
        },
        EmergencyMode::Frozen => {
            return Err(ProgramError::InvalidAccountData);
        }
    }
    
    Ok(())
}

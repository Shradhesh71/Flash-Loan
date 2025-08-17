use pinocchio::{
    account_info::AccountInfo, 
    program_error::ProgramError, 
    sysvars::{clock::Clock, Sysvar}, 
    ProgramResult
};

use crate::EmergencyState;

pub struct UnpauseAccounts<'a> {
    pub emergency_account: &'a AccountInfo,
    pub admin: &'a AccountInfo,
}

impl<'a> TryFrom<&'a [AccountInfo]> for UnpauseAccounts<'a> {
    type Error = ProgramError;
    
    fn try_from(accounts: &'a [AccountInfo]) -> Result<Self, Self::Error> {
        let [emergency_account, admin, ..] = accounts else {
            return Err(ProgramError::NotEnoughAccountKeys);
        };

        Ok(Self {
            emergency_account,
            admin,
        })
    }
}

pub struct Unpause<'a> {
    pub accounts: UnpauseAccounts<'a>,
}

impl<'a> TryFrom<&'a [AccountInfo]> for Unpause<'a> {
    type Error = ProgramError;
    
    fn try_from(accounts: &'a [AccountInfo]) -> Result<Self, Self::Error> {
        let accounts = UnpauseAccounts::try_from(accounts)?;
        
        Ok(Self { accounts })
    }
}

impl<'a> Unpause<'a> {
    pub const DISCRIMINATOR: &'a u8 = &12;
    
    pub fn process(&self) -> ProgramResult {
        // Load emergency state
        let emergency_data = self.accounts.emergency_account.try_borrow_data()?;
        let emergency_state = unsafe {
            *(emergency_data.as_ptr() as *const EmergencyState)
        };
        
        // Verify admin authority
        if emergency_state.admin != *self.accounts.admin.key() {
            return Err(ProgramError::InvalidAccountData);
        }
        
        // Update state to unpaused
        drop(emergency_data);
        let mut updated_state = emergency_state;
        updated_state.is_paused = false;
        updated_state.last_updated = Clock::get()?.unix_timestamp;
        
        // Write updated state
        let mut emergency_data = self.accounts.emergency_account.try_borrow_mut_data()?;
        let updated_state_bytes = unsafe {
            core::slice::from_raw_parts(
                &updated_state as *const EmergencyState as *const u8,
                core::mem::size_of::<EmergencyState>(),
            )
        };
        emergency_data[..core::mem::size_of::<EmergencyState>()].copy_from_slice(updated_state_bytes);
        
        Ok(())
    }
}


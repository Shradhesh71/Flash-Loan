use pinocchio::{
    account_info::AccountInfo, 
    program_error::ProgramError, 
    sysvars::{clock::Clock, Sysvar}, 
    ProgramResult
};

use crate::{EmergencyMode, EmergencyState};

pub struct SetEmergencyModeAccounts<'a> {
    pub emergency_account: &'a AccountInfo,
    pub admin: &'a AccountInfo,
}

impl<'a> TryFrom<&'a [AccountInfo]> for SetEmergencyModeAccounts<'a> {
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

pub struct SetEmergencyModeInstructionData {
    pub mode: EmergencyMode,
}

impl TryFrom<&[u8]> for SetEmergencyModeInstructionData {
    type Error = ProgramError;
    
    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        let mode_byte = data.get(0).ok_or(ProgramError::InvalidInstructionData)?;
        
        let mode = match *mode_byte {
            0 => EmergencyMode::Normal,
            1 => EmergencyMode::Limited,
            2 => EmergencyMode::Emergency,
            3 => EmergencyMode::Frozen,
            _ => return Err(ProgramError::InvalidInstructionData),
        };
        
        Ok(Self { mode })
    }
}

pub struct SetEmergencyMode<'a> {
    pub accounts: SetEmergencyModeAccounts<'a>,
    pub instruction_data: SetEmergencyModeInstructionData,
}

impl<'a> TryFrom<(&'a [u8], &'a [AccountInfo])> for SetEmergencyMode<'a> {
    type Error = ProgramError;
    
    fn try_from((data, accounts): (&'a [u8], &'a [AccountInfo])) -> Result<Self, Self::Error> {
        let accounts = SetEmergencyModeAccounts::try_from(accounts)?;
        let instruction_data = SetEmergencyModeInstructionData::try_from(data)?;
        
        Ok(Self {
            accounts,
            instruction_data,
        })
    }
}

impl<'a> SetEmergencyMode<'a> {
    pub const DISCRIMINATOR: &'a u8 = &13;
    
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
        
        // Update emergency mode
        drop(emergency_data);
        let clock = Clock::get()?;
        let mut updated_state = emergency_state;
        updated_state.emergency_mode = self.instruction_data.mode;
        updated_state.last_updated = clock.unix_timestamp;
        
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

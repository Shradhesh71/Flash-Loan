use pinocchio::pubkey::Pubkey;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct EmergencyState {
    pub is_paused: bool,                    // Global pause state
    pub admin: Pubkey,                      // Emergency admin
    pub max_loan_amount: u64,               // Maximum loan limit per transaction
    pub max_total_outstanding: u64,         // Maximum total loans outstanding
    pub emergency_mode: EmergencyMode,      // Current emergency level
    pub last_updated: i64,                  // Last update timestamp
    pub has_pending_admin: bool,            // Whether there's a pending admin transfer
    pub pending_admin: Pubkey,              // Pending admin transfer (only valid if has_pending_admin is true)
    pub admin_transfer_timestamp: i64,      // Admin transfer cooldown
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EmergencyMode {
    Normal = 0,        // Normal operations
    Limited = 1,       // Limited operations (reduced limits)
    Emergency = 2,     // Emergency mode (minimal operations)
    Frozen = 3,        // Completely frozen (only repay allowed)
}
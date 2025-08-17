pub mod repay;
pub mod loan;
pub mod helper;
pub mod emergency;

pub use helper::*;
pub use loan::*;
pub use repay::*;
pub use emergency::*;

pub const INITIALIZE_EMERGENCY: u8 = 10;
pub const PAUSE: u8 = 11;
pub const UNPAUSE: u8 = 12;
pub const SET_EMERGENCY_MODE: u8 = 13;
pub const UPDATE_LIMITS: u8 = 14;
pub const TRANSFER_ADMIN: u8 = 15;
pub const ACCEPT_ADMIN: u8 = 16;
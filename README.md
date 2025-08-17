# Pinocchio Flash Loan

A Solana flash loan program built with the Pinocchio framework. Enables instant loans that must be repaid within the same transaction with comprehensive emergency controls for enhanced security.

## Features

### Core Flash Loan Features
- **Flash Loans**: Borrow tokens instantly without collateral
- **Fee System**: Configurable fees (basis points)
- **Automatic Repayment**: Must repay + fee in same transaction
- **Protocol PDA**: Secure fund management via Program Derived Addresses

### Emergency Controls System
- **Emergency Pause**: Instantly halt all loan operations during security incidents
- **Emergency Modes**: Four-tier security system (Normal, Limited, Emergency, Frozen)
- **Admin Controls**: Secure administrative functions with time-locked transfers
- **Dynamic Limits**: Adjustable loan amounts based on emergency level

## Build

```bash
cargo build-sbf
```

## Test

### Flash Loan Tests
```bash
cargo test --test flash_loan
```

### Emergency Controls Tests
```bash
cargo test --test emergency_controls
```

### All Tests
```bash
cargo test
```

## Project Structure

```
src/
├── lib.rs              # Program entrypoint
├── state.rs            # Emergency state management
└── instructions/
    ├── loan.rs          # Flash loan logic
    ├── repay.rs         # Repay validation
    ├── helper.rs        # Shared utilities
    └── emergency/       # Emergency controls module
        ├── mod.rs       # Emergency module exports
        ├── initialize.rs # Initialize emergency system
        ├── pause.rs     # Emergency pause operations
        ├── unpause.rs   # Emergency unpause operations
        └── set_mode.rs  # Emergency mode management
tests/
├── flash_loan.rs       # Flash loan test suite
└── emergency_controls.rs # Emergency controls test suite
```

## Usage

### Flash Loan Operations
1. **Take Loan**: Borrow tokens from protocol
2. **Use Funds**: Execute your trading/arbitrage logic
3. **Repay**: Return borrowed amount + fee

All steps must complete in a single transaction or the entire operation fails.

### Emergency Controls

#### Admin Operations
- **Initialize Emergency System**: Set up emergency controls with admin
- **Pause/Unpause**: Instantly halt/resume all operations
- **Set Emergency Mode**: Change operational limits
- **Admin Transfer**: Time-locked admin transfers for security

## Dependencies

- `pinocchio` - Solana program framework
- `pinocchio-token` - SPL token operations
- `pinocchio-system` - System program interactions

## Author 

**Maintained by [@Shradhesh71](https://github.com/Shradhesh71)**  
---

**Built with ❤️ for the Solana ecosystem**
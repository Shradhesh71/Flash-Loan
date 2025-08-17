# Pinocchio Flash Loan

A Solana flash loan program built with the Pinocchio framework. Enables instant loans that must be repaid within the same transaction.

## Features

- **Flash Loans**: Borrow tokens instantly without collateral
- **Fee System**: Configurable fees (basis points)
- **Automatic Repayment**: Must repay + fee in same transaction
- **Protocol PDA**: Secure fund management via Program Derived Addresses

## Build

```bash
cargo build-sbf
```

## Test

```bash
cargo test --test flash_loan
```

**Test Results**: ✅ 6/6 tests passing

## Project Structure

```
src/
├── lib.rs              # Program entrypoint
└── instructions/
    ├── loan.rs          # Flash loan logic
    ├── repay.rs         # Repay validation
    └── helper.rs        # Shared utilities
tests/
└── flash_loan.rs       # Comprehensive test suite
```

## Usage

1. **Take Loan**: Borrow tokens from protocol
2. **Use Funds**: Execute your trading/arbitrage logic
3. **Repay**: Return borrowed amount + fee

All steps must complete in a single transaction or the entire operation fails.

## Dependencies

- `pinocchio` - Solana program framework
- `pinocchio-token` - SPL token operations
- `pinocchio-system` - System program interactions

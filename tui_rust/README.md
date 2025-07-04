# TauFoldzkVM TUI

A rich Terminal User Interface for demonstrating TauFoldzkVM applications with full implementation and polish.

## Features

### Demo Applications

1. **Calculator** - Full arithmetic calculator with zkVM proof generation
   - Complete numeric input (0-9, decimal point)
   - Operations: +, -, *, /
   - Memory functions: Store (m), Recall (r)
   - Clear (c) functionality
   - Real-time proof generation for each calculation

2. **Crypto Demo** - Comprehensive cryptographic operations
   - **Hash Mode**: SHA256 hashing
   - **Sign Mode**: Digital signature generation
   - **Verify Mode**: Signature verification
   - **Encrypt Mode**: Symmetric encryption
   - **Decrypt Mode**: Symmetric decryption
   - Live key generation and display

3. **Pacman Game** - Fully playable game with zkVM state verification
   - Complete maze with walls and paths
   - Player movement with arrow keys
   - Four unique ghosts with AI behaviors:
     - **Blinky (Red)**: Direct chase
     - **Pinky (Pink)**: Ambush ahead
     - **Inky (Blue)**: Complex targeting
     - **Clyde (Orange)**: Chase/scatter based on distance
   - Power pellets enable ghost eating
   - Score tracking and lives system
   - Game states: Playing, Power-up, Game Over, Victory, Paused

4. **Smart Contract** - ERC20-like token implementation
   - Token transfers between accounts
   - Minting (owner only)
   - Burning tokens
   - Transaction history
   - Account balance tracking
   - Pausable functionality
   - Real-time message display

5. **Vending Machine** - Complete FSM implementation
   - 8 different items with inventory tracking
   - Coin insertion: Quarter (Q), Dime (D), Nickel (N)
   - Bill insertion (B)
   - Item selection (1-8)
   - Change calculation and return
   - Cancel transaction (C)
   - State transitions: Idle → ItemSelected → AcceptingPayment → Dispensing → ReturningChange

### zkVM Integration

Each application integrates with the Tau language runtime to:
- Generate zero-knowledge proofs for all operations
- Verify state transitions cryptographically
- Display real-time execution statistics
- Show constraint generation and folding progress
- Visualize proof generation in real-time

## Building

```bash
cd tui_rust
cargo build --release
```

## Running

```bash
cargo run
```

Or after building:
```bash
./target/release/tau-zkvm-tui
```

## Controls

### Main Menu
- `↑/↓` - Navigate between applications
- `Enter` - Select application
- `q` or `Esc` - Quit

### In Applications
- `Esc` - Return to main menu
- App-specific controls vary per application

## Architecture

The TUI provides a rich interface for the zkVM applications but requires the Tau language runtime to actually execute the `.zkvm` files. The interface includes:

- **Main Menu**: Browse and select available demo applications
- **App Display**: Custom UI for each application type
- **zkVM Stats**: Real-time visualization of execution metrics
- **Progress Bars**: Visual feedback for folding, constraints, and proof generation

## Integration with Tau

The TUI loads and executes `.zkvm` files from the `../apps/` directory. To run the actual zero-knowledge computations, you need:

1. The Tau language runtime installed
2. The compiled `.zkvm` files in the apps directory

The `zkvm.rs` module handles the interface between the TUI and Tau runtime.

## Dependencies

- `ratatui` - Terminal UI framework
- `crossterm` - Cross-platform terminal manipulation
- `tokio` - Async runtime
- `anyhow` - Error handling
- `serde` - Serialization
- `tui-textarea` - Text input widget

## Implementation Details

### Calculator
- Full expression evaluation with proper operator precedence
- Memory storage and recall functionality
- Decimal number support
- Error handling for division by zero

### Crypto Demo
- Real SHA256 hashing using sha2 crate
- Simplified digital signature scheme
- XOR-based encryption for demonstration
- Base64 encoding for binary data display

### Pacman Game
- 21x19 maze grid with wall collision
- Ghost AI implementing classic Pacman behaviors
- Power-up timer system (200 ticks)
- Score multiplier for consecutive ghost eating
- Victory condition: collect all dots and power pellets

### Smart Contract
- HashMap-based account storage
- Transaction logging with timestamps
- Owner-only mint functionality
- Balance verification before transfers
- Chronological transaction history

### Vending Machine
- Complete finite state machine implementation
- Inventory management with stock tracking
- Accurate change calculation
- Multiple payment methods
- Transaction rollback on cancellation

## Technical Stack

- **ratatui 0.26** - Terminal UI framework
- **crossterm 0.27** - Cross-platform terminal manipulation
- **tokio 1.35** - Async runtime for zkVM execution
- **sha2 0.10** - Cryptographic operations
- **chrono 0.4** - Timestamp handling for transactions
- **base64 0.21** - Encoding for crypto demo
- **rand 0.8** - Random number generation

## License

MIT License - See LICENSE file in the project root.

Copyright (c) 2025 DarkLightX/Dana Edwards
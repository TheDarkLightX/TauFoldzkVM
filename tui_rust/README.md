# TauFoldzkVM TUI

A rich terminal user interface for interacting with TauFoldzkVM demo applications.

## Features

- 🎨 Beautiful terminal UI built with Ratatui
- 📱 Interactive demo applications:
  - Calculator - Basic arithmetic operations
  - Crypto Demo - Cryptographic operations
  - Pacman Game - Classic game implementation
  - Smart Contract - Contract execution example
  - Vending Machine - State machine demo
- 📊 Real-time zkVM execution visualization
- 📈 Live statistics display (cycles, constraints, folding steps)
- ⚡ Async execution with Tokio

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

## Future Enhancements

- [ ] Actual Tau runtime integration
- [ ] Interactive calculator with full numeric input
- [ ] Playable Pacman game with arrow key controls
- [ ] Smart contract parameter input
- [ ] Export execution proofs
- [ ] Network mode for distributed proving
#!/bin/bash

# TauFoldzkVM Setup Script
# Copyright (c) 2025 DarkLightX/Dana Edwards

set -e

echo "======================================"
echo "   TauFoldzkVM Setup & Installation   "
echo "======================================"
echo ""

# Colors for output
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Function to check if a command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Function to print success
print_success() {
    echo -e "${GREEN}✓${NC} $1"
}

# Function to print error
print_error() {
    echo -e "${RED}✗${NC} $1"
}

# Function to print warning
print_warning() {
    echo -e "${YELLOW}!${NC} $1"
}

echo "Checking system requirements..."
echo ""

# Check for Rust
if command_exists rustc; then
    RUST_VERSION=$(rustc --version | cut -d' ' -f2)
    print_success "Rust installed (version $RUST_VERSION)"
else
    print_error "Rust not found!"
    echo ""
    echo "Please install Rust from: https://rustup.rs/"
    echo "Run: curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    exit 1
fi

# Check for Cargo
if command_exists cargo; then
    print_success "Cargo installed"
else
    print_error "Cargo not found!"
    exit 1
fi

# Check Rust version (need 1.70+)
RUST_MAJOR=$(rustc --version | cut -d' ' -f2 | cut -d'.' -f1)
RUST_MINOR=$(rustc --version | cut -d' ' -f2 | cut -d'.' -f2)
if [ "$RUST_MAJOR" -gt 1 ] || ([ "$RUST_MAJOR" -eq 1 ] && [ "$RUST_MINOR" -ge 70 ]); then
    print_success "Rust version compatible"
else
    print_error "Rust version too old. Need 1.70+"
    echo "Run: rustup update"
    exit 1
fi

echo ""
echo "Checking for Tau language..."

# Check for Tau (optional but recommended)
if command_exists tau; then
    print_success "Tau language runtime found"
    TAU_INSTALLED=true
else
    print_warning "Tau language runtime not found"
    echo ""
    echo "The TUI will run in demo mode without Tau."
    echo "To enable full zkVM functionality, install Tau from:"
    echo "https://github.com/IDNI/tau-lang"
    echo ""
    TAU_INSTALLED=false
fi

echo ""
echo "Building TauFoldzkVM TUI..."
echo ""

# Change to TUI directory
cd tui_rust

# Clean previous builds
if [ -d "target" ]; then
    echo "Cleaning previous build..."
    cargo clean
fi

# Build in release mode
echo "Building release version..."
if cargo build --release; then
    print_success "Build completed successfully!"
else
    print_error "Build failed!"
    exit 1
fi

# Create convenient run script in project root
cd ..
cat > run.sh << 'EOF'
#!/bin/bash
# Quick run script for TauFoldzkVM TUI

# Colors
GREEN='\033[0;32m'
BLUE='\033[0;34m'
NC='\033[0m'

echo -e "${BLUE}Starting TauFoldzkVM TUI...${NC}"
echo ""

# Check if binary exists
if [ ! -f "tui_rust/target/release/tau-zkvm-tui" ]; then
    echo "TUI not built. Running setup first..."
    ./setup.sh
fi

# Run the TUI
cd tui_rust && ./target/release/tau-zkvm-tui "$@"
EOF

chmod +x run.sh

# Create demo mode script (without Tau)
cat > demo.sh << 'EOF'
#!/bin/bash
# Demo mode - runs without Tau runtime

export ZKVM_DEMO_MODE=true
./run.sh "$@"
EOF

chmod +x demo.sh

echo ""
echo "======================================"
echo "        Setup Complete! 🎉            "
echo "======================================"
echo ""
echo "To run TauFoldzkVM TUI:"
echo ""
print_success "./run.sh          - Run with Tau integration"
print_success "./demo.sh         - Run in demo mode (no Tau needed)"
echo ""
echo "Or run directly:"
echo "  cd tui_rust && cargo run"
echo ""

if [ "$TAU_INSTALLED" = false ]; then
    echo "Note: Running in demo mode since Tau is not installed."
    echo "Install Tau for full zkVM functionality."
fi

echo ""
echo "Enjoy exploring zero-knowledge applications!"
echo ""
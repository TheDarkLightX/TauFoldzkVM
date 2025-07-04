#!/bin/bash

# TauFoldzkVM One-Line Installer
# Copyright (c) 2025 DarkLightX/Dana Edwards

set -e

echo "========================================="
echo "   TauFoldzkVM One-Line Installation    "
echo "========================================="
echo ""

# Colors for output
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to print status
print_status() {
    echo -e "${BLUE}→${NC} $1"
}

print_success() {
    echo -e "${GREEN}✓${NC} $1"
}

print_error() {
    echo -e "${RED}✗${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}!${NC} $1"
}

# Get the directory where this script is located
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

print_status "Running setup script..."
if [ -f "$SCRIPT_DIR/setup.sh" ]; then
    bash "$SCRIPT_DIR/setup.sh"
else
    print_error "Setup script not found. Please ensure you're in the TauFoldzkVM directory."
    exit 1
fi

echo ""
echo "========================================="
echo "           Ready to Launch! 🚀          "
echo "========================================="
echo ""

print_success "Installation complete!"
echo ""
echo "Launch options:"
echo ""
echo -e "  ${GREEN}./run.sh${NC}    - Run with full zkVM integration"
echo -e "  ${GREEN}./demo.sh${NC}   - Run in demo mode (no dependencies)"
echo ""

# Auto-launch if user wants
echo -n "Would you like to launch the TUI now? [Y/n]: "
read -r response
if [[ "$response" =~ ^([yY][eE][sS]|[yY]|"")$ ]]; then
    echo ""
    print_status "Launching TauFoldzkVM TUI..."
    echo ""
    if [ -f "$SCRIPT_DIR/demo.sh" ]; then
        bash "$SCRIPT_DIR/demo.sh"
    else
        bash "$SCRIPT_DIR/run.sh"
    fi
else
    echo ""
    print_status "You can launch the TUI anytime with:"
    echo "  cd $(basename "$SCRIPT_DIR") && ./run.sh"
    echo ""
fi
#!/bin/bash
# Build script for TriageIR CLI on Unix-like systems

set -e  # Exit on any error

echo "Building TriageIR CLI..."

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo "Error: Rust/Cargo not found. Please install Rust from https://rustup.rs/"
    exit 1
fi

# Clean previous builds
echo "Cleaning previous builds..."
cargo clean

# Run tests first
echo "Running tests..."
cargo test

# Build debug version
echo "Building debug version..."
cargo build

# Build release version
echo "Building optimized release version..."
cargo build --release

echo ""
echo "Build completed successfully!"
echo ""
echo "Debug executable:   target/debug/triageir-cli"
echo "Release executable: target/release/triageir-cli"
echo ""
echo "To test the executable:"
echo "  ./target/release/triageir-cli --help"
echo "  ./target/release/triageir-cli --verbose --output test_results.json"
echo ""
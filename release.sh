#!/bin/bash

set -e  # Exit immediately if a command exits with a non-zero status

# Define variables
PROJECT_DIR="$(pwd)"
TARGET_BINARY="rsp"
BUILD_DIR="target/release"
TARGET_ARCH=${TARGET_ARCH:-x86_64-unknown-linux-gnu}


# Build the Rust project in release mode
echo "Building the Rust project..."
cargo build --release --target $TARGET_ARCH

echo "Build complete. Run './$BUILD_DIR/$TARGET_BINARY' to execute."
cp "$BUILD_DIR/$TARGET_BINARY" "$(pwd)/"
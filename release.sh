#!/bin/bash

# Detect system architecture
ARCH=$(uname -m)

# Validate architecture
case "$ARCH" in
    x86_64|aarch64|armv7l|i686)
        echo "Building for detected architecture: $ARCH"
        ;;
    *)
        echo "Unsupported architecture: $ARCH"
        exit 1
        ;;
esac

# Build using the default target (native system)
cargo build --release

echo "Build completed for $ARCH"

# copy the binary to the current directory
cp target/release/rsp .

echo "Binary copied to current directory"

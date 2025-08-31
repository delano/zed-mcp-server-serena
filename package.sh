#!/bin/bash

# Serena Context Server Extension Packaging Script

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${GREEN}Building Serena Context Server Extension...${NC}"

# Build the extension for WebAssembly (Zed extensions)
cargo build --target wasm32-wasip1 --release

echo -e "${GREEN}Build completed successfully!${NC}"

# Package the extension
echo -e "${YELLOW}Packaging extension...${NC}"

# Create package directory
PKG_DIR="serena-context-server"
rm -rf "$PKG_DIR"
mkdir "$PKG_DIR"

# Copy necessary files
cp extension.toml "$PKG_DIR/"
cp README.md "$PKG_DIR/"
cp target/wasm32-wasip1/release/*.wasm "$PKG_DIR/"

echo -e "${GREEN}Extension packaged in ${PKG_DIR}/${NC}"

# Instructions for installation
echo -e "${YELLOW}To install this extension in Zed:${NC}"
echo "1. Copy the ${PKG_DIR} folder to your Zed extensions directory:"
echo "   - macOS: ~/.config/zed/extensions/"
echo "   - Linux: ~/.config/zed/extensions/"
echo "   - Windows: %APPDATA%/Zed/extensions/"
echo ""
echo "2. Or install via Zed's extension manager when published"
echo ""
echo -e "${GREEN}Extension is ready for use!${NC}"
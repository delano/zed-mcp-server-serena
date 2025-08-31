# Developing the Serena Context Server Extension for Zed

This document provides detailed instructions for building, testing, and contributing to the Serena Context Server extension for Zed.

## Development Environment Setup

### Prerequisites

- **Rust**: Latest stable version with `wasm32-wasip1` target
- **Python**: 3.11 or later
- **Zed Editor**: Latest version
- **Git**: For version control

### Initial Setup

1. **Clone the repository:**
   ```bash
   git clone <your-repo-url>
   cd zed-mcp-server-serena
   ```

2. **Install Rust and WebAssembly target:**
   ```bash
   # Install Rust if not already installed
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source ~/.cargo/env

   # Add WebAssembly target
   rustup target add wasm32-wasip1
   ```

3. **Verify Python installation:**
   ```bash
   python3 --version  # Should be 3.11+
   pip3 --version
   ```

## Building the Extension

### Standard Build

```bash
# Clean build (recommended)
cargo clean

# Build for WebAssembly (release mode recommended)
cargo build --target wasm32-wasip1 --release
```

The compiled extension will be located at:
`target/wasm32-wasip1/release/zed_serena_context_server.wasm`

### Development Build

For faster compilation during development:

```bash
cargo build --target wasm32-wasip1
```

### Troubleshooting Build Issues

#### Edition 2024 Dependency Error

If you encounter errors related to `edition2024`:

```bash
# Try using nightly toolchain
rustup toolchain install nightly
rustup target add wasm32-wasip1 --toolchain nightly
cargo +nightly build --target wasm32-wasip1 --release
```

#### Registry Cache Issues

```bash
# Clear cargo cache and retry
rm -rf ~/.cargo/registry/cache
rm -rf ~/.cargo/registry/src
cargo clean
cargo build --target wasm32-wasip1 --release
```

#### Rust Version Issues

```bash
# Update Rust to latest
rustup self update
rustup update stable
```

## Testing the Extension

### 1. Install as Development Extension

```bash
# Build first
cargo build --target wasm32-wasip1 --release

# In Zed:
# 1. Open Command Palette (Cmd+Shift+P)
# 2. Run "zed: install dev extension"
# 3. Select this directory: /path/to/zed-mcp-server-serena
```

### 2. Automatic Configuration

**No manual configuration required!** The extension automatically:
- Detects Python 3.11/3.12 on your system
- Installs Serena via pip if not present
- Registers the context server in Zed
- Provides 19 MCP tools immediately

### 3. Manual Configuration (Fallback Only)

Only needed if automatic setup fails:

```json
{
  "context_servers": {
    "serena-context-server": {
      "command": "/opt/homebrew/bin/serena",
      "args": ["start-mcp-server"],
      "env": {
        "SERENA_LOG_LEVEL": "debug"
      }
    }
  }
}
```

**Note**: Use full Python path for reliability. The extension prioritizes:
1. `/opt/homebrew/bin/python3.11` (macOS Homebrew)
2. `/usr/local/bin/python3.11` (standard locations)
3. `python3.11`, `python3.12` in PATH

**Available Tools:** Serena automatically exposes 19 MCP tools for semantic code analysis:

- **File Operations**: `list_dir`, `find_file`, `search_for_pattern`
- **Semantic Analysis**: `get_symbols_overview`, `find_symbol`, `find_referencing_symbols`
- **Code Manipulation**: `replace_symbol_body`, `insert_after_symbol`, `insert_before_symbol`
- **Memory Management**: `write_memory`, `read_memory`, `list_memories`, `delete_memory`
- **Agent Workflow**: `onboarding`, `think_about_collected_information`, `think_about_task_adherence`, `think_about_whether_you_are_done`, `check_onboarding_performed`, `initial_instructions`

### 4. Verification

To verify the extension is working:

```
Ask your AI assistant: "List the MCP tools available to you right now"
```

Expected result: 19 Serena tools should appear, including semantic analysis and code manipulation capabilities.

### 5. Verify Installation

1. **Check Extension Panel:**
   - Go to Extensions in Zed
   - Look for "Serena Context Server" with ✅ status

2. **Test AI Integration:**
   - Ask: "List the MCP tools available to you right now"
   - Should show 19 Serena tools

3. **Test Semantic Analysis:**
   - Ask: "Use Serena to analyze the symbols in this project"
   - Should provide symbol overview and analysis

### 4. Manual Testing

Test the underlying Serena installation:

```bash
# Install Serena manually
pip install serena-agent

# Test MCP server
serena start-mcp-server

# Should start without errors
```

## Development Workflow

### Code Structure

```
zed-mcp-server-serena/
├── Cargo.toml           # Rust package configuration
├── extension.toml       # Zed extension metadata
├── src/
│   └── lib.rs          # Main extension implementation
├── README.md           # Project overview
└── DEVELOPING.md       # This file
```

### Key Components

- **`extension.toml`**: Extension metadata and MCP server registration
- **`Cargo.toml`**: Rust dependencies and build configuration
- **`src/lib.rs`**: Main extension logic including:
  - Python detection
  - Serena installation
  - MCP server command generation
  - Error handling

### Making Changes

1. **Edit the code** in `src/lib.rs`
2. **Rebuild** with `cargo build --target wasm32-wasip1 --release`
3. **Reinstall** the dev extension in Zed
4. **Test** the changes

### Extension API Usage

The extension uses the Zed Extension API:

- `context_server_command()`: Returns command to start MCP server
- `context_server_configuration()`: Provides setup instructions and schema
- Error handling via `Result<T>` types
- Cross-platform path handling

## Testing Strategies

### Unit Testing

```bash
# Run Rust tests
cargo test
```

### Integration Testing

1. **Test Python Detection:**
   ```bash
   # Test different Python versions
   python3.11 --version
   python3.12 --version
   python3 --version
   ```

2. **Test Serena Installation:**
   ```bash
   # Test pip installation
   pip install serena-agent
   python -c "import serena; print('OK')"
   ```

3. **Test MCP Server:**
   ```bash
   # Test server startup
   serena start-mcp-server
   # Should start without immediate exit
   ```

### End-to-End Testing

1. **Fresh Environment Test:**
   - Use a clean Python environment
   - Uninstall Serena: `pip uninstall serena-agent`
   - Test extension auto-installation

2. **Cross-Platform Testing:**
   - Test on macOS, Linux, Windows
   - Verify path handling works correctly

## Debugging

### Extension Logs

Check Zed's debug output for:
- Extension loading messages
- Python detection results (should find `/opt/homebrew/bin/python3.11`)
- Serena installation progress
- MCP server startup errors

### Common Issues & Solutions

#### 1. Extension Not Loading
**Problem**: Extension doesn't appear in Zed Extensions panel
**Solutions**:
- ✅ **Use Zed Preview/Dev**: Extensions don't work in Zed Stable
- ✅ **Install Correct Directory**: Use `serena-context-server/` not root directory
- ✅ **Rebuild Extension**: `cargo build --target wasm32-wasip1 --release`
- ✅ **Update WASM**: `cp target/wasm32-wasip1/release/zed_serena_context_server.wasm extension.wasm`

#### 2. Python Version Issues
**Problem**: "Python 3.11 or 3.12 not found" error
**Solutions**:
```bash
# Install compatible Python
brew install python@3.11

# Verify installation
python3.11 --version
which python3.11

# Test Serena installation
python3.11 -m pip install serena-agent
python3.11 -c "import serena; print('Version:', serena.__version__)"
```

#### 3. Serena Installation Fails
**Problem**: Extension can't install `serena-agent`
**Solutions**:
- Check pip permissions and internet connection
- Install manually: `python3.11 -m pip install serena-agent`
- Use virtual environment if needed
- Verify Python 3.11/3.12 specifically (3.13+ not supported)

#### 4. MCP Server Won't Start
**Problem**: Context server shows error status
**Solutions**:
```bash
# Test MCP server manually
serena start-mcp-server

# Check if command exists
serena --help

# Verify imports work
python3.11 -c "import serena.cli; print('OK')"
```

#### 5. No Serena Tools Available
**Problem**: AI assistant doesn't see Serena tools
**Solutions**:
1. Check Extensions panel shows "Serena Context Server" with ✅
2. Restart Zed completely
3. Ask AI: "List available MCP tools" - should show 19 Serena tools
4. Try manual configuration as fallback (see above)

#### 6. Automatic Detection Fails
**Problem**: Extension can't find Python/Serena automatically
**Manual Configuration**:
```json
{
  "context_servers": {
    "serena-context-server": {
      "command": "/opt/homebrew/bin/serena",
      "args": ["start-mcp-server"],
      "env": {
        "SERENA_LOG_LEVEL": "debug"
      }
    }
  }
}
```

### Debugging Commands

```bash
# Check Python setup
which python3
python3 --version
python3 -c "import sys; print(sys.path)"

# Check Serena installation
pip list | grep serena
python3 -c "import serena; print(serena.__file__)"

# Test MCP server manually
serena --help
serena start-mcp-server --help
```

## Contributing

### Code Style

- Follow Rust standard formatting: `cargo fmt`
- Run linter: `cargo clippy`
- Keep code well-documented with comments

### Pull Request Process

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Test thoroughly (build + manual testing)
5. Submit a pull request with:
   - Clear description of changes
   - Test results
   - Any breaking changes noted

### Reporting Issues

Include in bug reports:
- Zed version
- Operating system
- Python version
- Extension version
- Error messages
- Steps to reproduce

## Release Process

1. **Update Version Numbers:**
   - `extension.toml`: version field
   - `Cargo.toml`: version field

2. **Test Release Build:**
   ```bash
   cargo build --target wasm32-wasip1 --release
   # Test installation and functionality
   ```

3. **Create Release:**
   - Tag version in git
   - Create release notes
   - Upload to Zed extension marketplace (when available)

## Additional Resources

- [Zed Extension Documentation](https://zed.dev/docs/extensions)
- [Serena Documentation](https://github.com/oraios/serena)
- [Model Context Protocol Specification](https://modelcontextprotocol.io/)
- [WebAssembly Target Documentation](https://doc.rust-lang.org/rustc/platform-support/wasm32-wasip1.html)

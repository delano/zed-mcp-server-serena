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
   cd mcp-server-serena
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
# 3. Select this directory: /path/to/mcp-server-serena
```

### 2. Configure Zed Settings

Add to your Zed settings (`Cmd+,`):

```json
{
  "context_servers": {
    "serena-context-server": {
      "source": "extension",
      "env": {
        "SERENA_LOG_LEVEL": "debug"
      }
    }
  }
}
```

For manual installation (without the extension), use:

```json
{
  "context_servers": {
    "serena-context-server": {
      "source": "custom",
      "command": "/usr/local/bin/python3",
      "args": ["-m", "serena.cli", "start_mcp_server"],
      "env": {
        "SERENA_LOG_LEVEL": "debug"
      }
    }
  }
}
```

**Available Tools:** Serena automatically exposes 19 MCP tools for semantic code analysis:

- **File Operations**: `list_dir`, `find_file`, `search_for_pattern`
- **Semantic Analysis**: `get_symbols_overview`, `find_symbol`, `find_referencing_symbols`
- **Code Manipulation**: `replace_symbol_body`, `insert_after_symbol`, `insert_before_symbol`
- **Memory Management**: `write_memory`, `read_memory`, `list_memories`, `delete_memory`
- **Agent Workflow**: `check_onboarding_performed`, `onboarding`, `think_about_collected_information`, `think_about_task_adherence`, `think_about_whether_you_are_done`, `initial_instructions`

The extension configuration uses `"source": "extension"` to let the extension manage the server startup. The manual configuration provides the direct command and arguments for running Serena's MCP server.

### 3. Verify Installation

1. **Check Extension Panel:**
   - Go to Extensions in Zed
   - Look for "Serena Context Server"

2. **Check Logs:**
   - Open Zed's debug console
   - Look for Serena-related messages

3. **Test Functionality:**
   - Open a code project
   - Try AI features - should use Serena's capabilities

### 4. Manual Testing

Test the underlying Serena installation:

```bash
# Install Serena manually
pip install serena-agent

# Test MCP server
python -m serena.cli start_mcp_server

# Should start without errors
```

## Development Workflow

### Code Structure

```
mcp-server-serena/
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
   python -m serena.cli start_mcp_server
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
- Python detection results
- Serena installation progress
- MCP server startup errors

### Common Issues

1. **Python Not Found:**
   - Verify Python 3.11+ is installed
   - Check PATH environment variable
   - Set explicit `python_executable` in settings

2. **Serena Installation Fails:**
   - Check pip permissions
   - Verify internet connection
   - Try manual installation

3. **MCP Server Won't Start:**
   - Check Python can import serena
   - Verify serena-mcp-server command exists
   - Check for port conflicts

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
python3 -m serena.cli --help
python3 -m serena.cli start_mcp_server --help
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

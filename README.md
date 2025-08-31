# Serena Context Server Extension for Zed

A Zed editor extension that integrates [Serena](https://github.com/oraios/serena) - a powerful coding agent toolkit with semantic code analysis capabilities - via the Model Context Protocol (MCP).

## About Serena

Serena is a free & open-source coding agent toolkit that provides semantic code retrieval and editing tools. It works like an IDE for coding agents, enabling them to:

- Find and edit code at the symbol level using tools like `find_symbol`, `find_referencing_symbols`, and `insert_after_symbol`
- Perform efficient semantic code analysis using Language Server Protocol (LSP)
- Work with large codebases without reading entire files or performing basic grep searches
- Support 20+ programming languages out of the box

## Features

This Zed extension provides seamless integration with Serena's MCP server, offering:

- **Automatic Installation**: Downloads and installs `serena-agent` via pip if not present
- **Smart Python Detection**: Automatically finds Python 3.11+ interpreter
- **Cross-Platform Support**: Works on macOS, Linux, and Windows
- **Configurable**: Supports custom Python paths and environment variables
- **Error Handling**: Clear error messages for missing dependencies

## Installation

### Prerequisites

- Python 3.11 or later
- pip package manager
- Rust toolchain (for building the extension)

### Building and Installing

See [DEVELOPING.md](./DEVELOPING.md) for detailed development and installation instructions.

### Quick Install (Once Available)

When published to the Zed extension marketplace:

1. Open Zed
2. Go to Extensions (`Cmd+Shift+X`)
3. Search for "Serena Context Server"
4. Click Install

## Configuration

The extension can be configured in your Zed settings (`Cmd+,`):

```json
{
  "context_servers": {
    "serena-context-server": {
      "settings": {
        // Optional: Specify Python executable path
        "python_executable": "/usr/local/bin/python3",
        
        // Optional: Additional environment variables
        "environment": {
          "SERENA_LOG_LEVEL": "debug"
        }
      }
    }
  }
}
```

## Usage

Once installed and configured:

1. Open any project in Zed
2. The extension will automatically start Serena's MCP server
3. AI features in Zed will now have access to Serena's semantic code analysis tools
4. Enjoy enhanced coding assistance with symbol-level precision!

## Supported Languages

Serena supports semantic analysis for:

- Python
- TypeScript/JavaScript  
- PHP
- Go
- R
- Rust
- C/C++
- Zig
- C#
- Ruby
- Swift
- Kotlin
- Java
- Clojure
- Dart
- Bash
- Lua
- Nix
- Elixir
- Erlang

## Development

See [DEVELOPING.md](./DEVELOPING.md) for:

- Building the extension from source
- Testing locally
- Contributing guidelines
- Troubleshooting common issues

## Related Projects

- [Serena](https://github.com/oraios/serena) - The main Serena coding agent toolkit
- [Zed](https://zed.dev) - The Zed code editor
- [Model Context Protocol](https://modelcontextprotocol.io/) - The protocol enabling AI-editor integration

## License

MIT

## Contributing

Contributions are welcome! Please see [DEVELOPING.md](./DEVELOPING.md) for development setup and contribution guidelines.
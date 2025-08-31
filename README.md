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
- **Smart Python Detection**: Automatically finds Python 3.11-3.12 interpreter (required by Serena)
- **Zero Configuration**: Works out-of-the-box without manual server setup
- **Cross-Platform Support**: Works on macOS, Linux, and Windows
- **19 Semantic Tools**: Complete toolkit for AI-powered code analysis and manipulation
- **Error Handling**: Clear error messages for missing dependencies

## Installation

### Prerequisites

- **Zed Preview/Dev**: Extension system requires Zed Preview or Dev channel (not Stable)
- **Python 3.11-3.12**: Serena specifically requires Python 3.11 or 3.12
- **pip**: Package manager for Python dependencies

### Quick Install Steps

1. **Install Compatible Python** (if needed):
   ```bash
   # macOS with Homebrew
   brew install python@3.11
   
   # Verify installation
   python3.11 --version
   ```

2. **Install as Dev Extension**:
   - Download/clone this repository
   - In Zed: `Cmd+Shift+P` → "zed: install dev extension"
   - Select: `path/to/zed-mcp-server-serena/serena-context-server` directory
   - The extension installs automatically without manual configuration

3. **Verify Installation**:
   - Check Extensions panel in Zed for "Serena Context Server"
   - No manual server configuration needed - it's handled automatically!

### Published Extension (Coming Soon)

When published to the Zed extension marketplace:

1. Open Zed
2. Go to Extensions (`Cmd+Shift+X`)
3. Search for "Serena Context Server"
4. Click Install

## Usage

### Automatic Operation

Once installed, the extension works automatically:

1. **Automatic Setup**: Extension detects Python 3.11/3.12 and installs Serena if needed
2. **Zero Configuration**: Context server registers automatically in Zed
3. **Ready to Use**: AI features immediately gain access to 19 Serena tools

### Testing Serena Integration

To verify Serena is working, ask your AI assistant in Zed:

```
List the MCP tools available to you right now
```

You should see 19 Serena tools including:
- `get_symbols_overview`, `find_symbol`, `find_referencing_symbols`
- `list_dir`, `find_file`, `search_for_pattern`
- `replace_symbol_body`, `insert_after_symbol`, `insert_before_symbol`
- `write_memory`, `read_memory`, `list_memories`, `delete_memory`
- Plus workflow and analysis tools

### Example Usage

**Semantic Code Analysis:**
```
Use Serena to analyze the symbols in this project and show me an overview
```

**Symbol Search:**
```
Find all functions related to "authentication" in this codebase using Serena
```

**Code Manipulation:**
```
Use Serena to add error handling to the login function
```

## Available Serena Tools

The extension provides 19 specialized MCP tools:

### 📁 File Operations
- `list_dir` - List directory contents
- `find_file` - Locate files by name/pattern
- `search_for_pattern` - Search file contents with regex

### 🔍 Semantic Analysis  
- `get_symbols_overview` - Get project symbol summary
- `find_symbol` - Locate specific symbols (functions, classes, etc.)
- `find_referencing_symbols` - Find references to symbols

### ✏️ Code Manipulation
- `replace_symbol_body` - Replace function/class implementations
- `insert_after_symbol` - Add code after symbols
- `insert_before_symbol` - Add code before symbols

### 🧠 Memory Management
- `write_memory` - Store information for later use
- `read_memory` - Retrieve stored information
- `list_memories` - List all stored information
- `delete_memory` - Remove stored information

### 🤖 Agent Workflow
- `onboarding` - Initialize project understanding
- `think_about_collected_information` - Analyze gathered data
- `think_about_task_adherence` - Verify task completion
- `think_about_whether_you_are_done` - Assess completion status
- `check_onboarding_performed` - Verify initialization
- `initial_instructions` - Get project context

## Configuration (Optional)

The extension works without configuration, but you can customize it in Zed settings (`Cmd+,`):

```json
{
  "context_servers": {
    "serena-context-server": {
      "settings": {
        // Optional: Specify Python executable path
        "python_executable": "/opt/homebrew/bin/python3.11",
        
        // Optional: Additional environment variables
        "environment": {
          "SERENA_LOG_LEVEL": "debug"
        }
      }
    }
  }
}
```

**Note**: Manual configuration is only needed if automatic detection fails.

## Troubleshooting

### Extension Not Loading
- ✅ **Use Zed Preview/Dev**: Extensions don't work in Zed Stable
- ✅ **Check Python Version**: Serena requires Python 3.11-3.12 specifically
- ✅ **Verify Path**: Install to `serena-context-server/` directory, not root

### Python Detection Issues
```bash
# Test Python detection manually
python3.11 --version
python3.11 -c "import serena; print('Serena:', serena.__version__)"

# Install Serena manually if needed
python3.11 -m pip install serena-agent
```

### Missing Tools
If Serena tools don't appear:
1. Check Extensions panel shows "Serena Context Server" with ✅
2. Restart Zed completely
3. Ask AI: "List available MCP tools" to verify

### Manual Installation Fallback
If automatic setup fails, add manual configuration:
```json
{
  "serena-context-server": {
    "command": "/opt/homebrew/bin/serena",
    "args": ["start-mcp-server"],
    "env": {
      "SERENA_LOG_LEVEL": "debug"
    }
  }
}
```

## Supported Languages

Serena provides semantic analysis for 20+ languages:

**Primary Support**: Python, TypeScript/JavaScript, PHP, Go, Rust, C/C++, Java, C#, Swift
**Additional**: Ruby, Kotlin, Clojure, Dart, Bash, Lua, Nix, Elixir, Erlang, Zig, R

## Development

See [DEVELOPING.md](./DEVELOPING.md) for:

- Building the extension from source
- Testing locally  
- Contributing guidelines
- Detailed troubleshooting

## Related Projects

- [Serena](https://github.com/oraios/serena) - The main Serena coding agent toolkit
- [Zed](https://zed.dev) - The Zed code editor
- [Model Context Protocol](https://modelcontextprotocol.io/) - The protocol enabling AI-editor integration

## License

MIT

## Contributing

Contributions are welcome! Please see [DEVELOPING.md](./DEVELOPING.md) for development setup and contribution guidelines.
# Quick Start Guide - Serena Context Server for Zed

## 🚀 Installation (3 Steps)

1. **Install Python 3.11/3.12** (if needed):
   ```bash
   brew install python@3.11
   python3.11 --version
   ```

2. **Install Extension in Zed**:
   - `Cmd+Shift+P` → "zed: install dev extension"
   - Select: `path/to/mcp-server-serena/serena-context-server`
   - ✅ Done! Zero configuration needed.

3. **Verify Working**:
   Ask AI: `"List the MCP tools available to you right now"`
   
   Expected: 19 Serena tools including `get_symbols_overview`, `find_symbol`, etc.

## 🧪 Test Commands

**Semantic Analysis:**
```
Use Serena to analyze the symbols in this project
```

**Symbol Search:**
```  
Find all functions related to "authentication" using Serena
```

**Code Manipulation:**
```
Use Serena to add error handling to the login function
```

## 🛠️ Troubleshooting

| Problem | Solution |
|---------|----------|
| Extension not loading | Use Zed Preview/Dev (not Stable) |
| Python version error | Install Python 3.11: `brew install python@3.11` |
| No Serena tools | Check Extensions panel for ✅ status, restart Zed |
| Manual config needed | See README.md for fallback configuration |

## 📋 19 Available Tools

### 📁 File Operations
`list_dir`, `find_file`, `search_for_pattern`

### 🔍 Semantic Analysis  
`get_symbols_overview`, `find_symbol`, `find_referencing_symbols`

### ✏️ Code Manipulation
`replace_symbol_body`, `insert_after_symbol`, `insert_before_symbol`

### 🧠 Memory Management
`write_memory`, `read_memory`, `list_memories`, `delete_memory`

### 🤖 Agent Workflow
`onboarding`, `think_about_collected_information`, `think_about_task_adherence`, `think_about_whether_you_are_done`, `check_onboarding_performed`, `initial_instructions`

---

**Need help?** See [README.md](./README.md) or [DEVELOPING.md](./DEVELOPING.md) for detailed documentation.
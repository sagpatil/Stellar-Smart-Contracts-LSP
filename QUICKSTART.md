# Quick Start Guide - Stellar Smart Contracts LSP

## 🚀 5-Minute Setup

### 1. Install Extension
```bash
# Download stellar-contracts-lsp-0.0.1.vsix from GitHub releases
code --install-extension stellar-contracts-lsp-0.0.1.vsix
```

### 2. Create New Project
```bash
stellar contract init my-contract
cd my-contract
code .
```

### 3. Start Coding
Open `src/lib.rs` and replace with:

```rust
#![no_std]
use soroban_sdk::{contract, contractimpl, Env, Symbol};

#[contract]
pub struct HelloWorld;

#[contractimpl]
impl HelloWorld {
    pub fn hello(env: Env, name: Symbol) -> Symbol {
        Symbol::new(&env, "Hello")
    }
}
```

### 4. Try Extension Features

**Auto-completion:**
- Type `#[con` → Press Tab → Gets `#[contract]`
- Type `env.` → See all available methods

**Hover Info:**
- Hover over `Env` to see documentation
- Hover over `Symbol` for type information

**Build & Test:**
- Press `Ctrl+Shift+P` (Cmd+Shift+P on Mac)
- Type "Stellar: Build" → Select "Build Stellar Contract"
- Type "Stellar: Test" → Select "Test Stellar Contract"

## 📝 Common Code Patterns

### Storage Operations
```rust
// Set storage
env.storage().instance().set(&key, &value);

// Get storage
let value: u32 = env.storage().instance().get(&key).unwrap_or(0);
```

### Authentication
```rust
pub fn protected_function(env: Env, user: Address) {
    user.require_auth();
    // Function logic here
}
```

### Contract Invocation
```rust
let result = env.invoke_contract(
    &contract_address,
    &Symbol::new(&env, "function_name"),
    args
);
```

## 🎯 Key Commands

| Command | Shortcut | Purpose |
|---------|----------|---------|
| Command Palette | `Ctrl+Shift+P` | Access all Stellar commands |
| Build Contract | Search "Stellar: Build" | Compile to WASM |
| Test Contract | Search "Stellar: Test" | Run tests |
| Deploy Contract | Search "Stellar: Deploy" | Deploy to network |
| Restart LSP | Search "Stellar: Restart" | Fix LSP issues |

## 🔧 Extension Features You'll Use

✅ **Smart Completions** - Type faster with auto-complete  
✅ **Error Detection** - Catch issues before compilation  
✅ **Hover Documentation** - Learn APIs without leaving editor  
✅ **CLI Integration** - Build, test, deploy from VS Code  
✅ **Syntax Highlighting** - Clear code visualization  

## 📚 Learn More

- **Full Guide**: See `USAGE_GUIDE.md` for complete documentation
- **Examples**: Check `test-contracts/` directory for advanced patterns
- **Stellar Docs**: https://developers.stellar.org/docs/build/smart-contracts/

---

**Ready to build on Stellar!** 🌟

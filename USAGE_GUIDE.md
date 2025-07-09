# Stellar Smart Contracts LSP - Usage Guide

A comprehensive guide for new users to get started with the Stellar Smart Contracts LSP extension.

## 📋 Prerequisites

Before using the extension, ensure you have:

1. **VS Code**: Version 1.101.0 or higher
2. **Rust Toolchain**: Install from [rustup.rs](https://rustup.rs/)
3. **Stellar CLI**: Install from [Stellar CLI Installation Guide](https://developers.stellar.org/docs/build/smart-contracts/getting-started/setup)

### Quick Installation Check

```bash
# Check Rust installation
rustc --version

# Check Stellar CLI installation
stellar --version

# Check VS Code version
code --version
```

## 🚀 Getting Started

### Step 1: Install the Extension

1. Download `stellar-contracts-lsp-0.0.1.vsix` from the [GitHub Release](https://github.com/sagpatil/Stellar-Smart-Contracts-LSP/releases/tag/v0.0.1)
2. Open VS Code
3. Go to Extensions view (`Ctrl+Shift+X` or `Cmd+Shift+X`)
4. Click the "..." menu and select "Install from VSIX..."
5. Select the downloaded `.vsix` file
6. Restart VS Code when prompted

### Step 2: Create Your First Stellar Contract

1. **Create a new directory for your project:**
   ```bash
   mkdir my-stellar-contract
   cd my-stellar-contract
   ```

2. **Initialize a new Stellar contract project:**
   ```bash
   stellar contract init my-contract
   cd my-contract
   ```

3. **Open the project in VS Code:**
   ```bash
   code .
   ```

### Step 3: Explore the Extension Features

Once you open a `.rs` file, the extension will automatically activate and provide:

## 🔧 Core Features Walkthrough

### 1. **Intelligent Code Completion**

Open `src/lib.rs` and start typing. The extension provides smart completions for:

**Contract Attributes:**
- Type `#[con` → Auto-completes to `#[contract]`
- Type `#[contractimpl` → Auto-completes to `#[contractimpl]`
- Type `#[contracttype` → Auto-completes to `#[contracttype]`

**Stellar Types:**
- Type `Env` → Shows completion with documentation
- Type `Address` → Auto-completes with type information
- Type `Symbol` → Provides Stellar-specific Symbol type

**Example Contract Structure:**
```rust
#![no_std]
use soroban_sdk::{contract, contractimpl, Env, Symbol, Address};

#[contract]
pub struct MyContract;

#[contractimpl]
impl MyContract {
    pub fn hello(env: Env, user: Address) -> Symbol {
        // Type 'env.' to see available methods
        env.storage().instance().set(&Symbol::new(&env, "user"), &user);
        Symbol::new(&env, "Hello")
    }
}
```

### 2. **Real-time Diagnostics**

The extension provides comprehensive error checking:

**Missing Attributes:**
- If you forget `#[contract]`, you'll see: "Missing #[contract] attribute"
- Missing `#[contractimpl]` will show appropriate warnings

**Authentication Checks:**
- Functions accessing storage will suggest authentication requirements
- Hover over functions to see security recommendations

**Best Practices:**
- Suggestions for proper error handling
- Recommendations for storage patterns

### 3. **Rich Hover Information**

Hover over any Stellar type or function to see:

**For `Env`:**
```
Env - The Stellar contract environment
Provides access to:
- Storage operations
- Authentication and authorization
- Contract invocation
- Cryptographic functions
```

**For `Address`:**
```
Address - Stellar account or contract address
Used for:
- Identifying accounts and contracts
- Authentication and authorization
- Contract invocation targets
```

### 4. **Stellar CLI Integration**

The extension integrates with Stellar CLI through VS Code commands:

**Access commands via:**
- Command Palette (`Ctrl+Shift+P` or `Cmd+Shift+P`)
- Type "Stellar" to see available commands

**Available Commands:**

1. **Build Contract:**
   - Command: `Stellar: Build Stellar Contract`
   - Runs: `stellar contract build`
   - Compiles your contract to WASM

2. **Test Contract:**
   - Command: `Stellar: Test Stellar Contract`
   - Runs: `stellar contract test`
   - Executes your contract tests

3. **Deploy Contract:**
   - Command: `Stellar: Deploy Stellar Contract`
   - Guides you through contract deployment

4. **Invoke Contract Function:**
   - Command: `Stellar: Invoke Stellar Contract Function`
   - Calls deployed contract functions

## 📝 Example: Complete Contract Development

Let's create a simple counter contract step by step:

### Step 1: Create the Contract

Replace the contents of `src/lib.rs`:

```rust
#![no_std]
use soroban_sdk::{contract, contractimpl, Env, Symbol, Address};

const COUNTER: Symbol = Symbol::new(&env, "COUNTER");

#[contract]
pub struct Counter;

#[contractimpl]
impl Counter {
    pub fn increment(env: Env, user: Address) -> u32 {
        user.require_auth();
        
        let current: u32 = env.storage().instance()
            .get(&COUNTER)
            .unwrap_or(0);
        
        let new_value = current + 1;
        env.storage().instance().set(&COUNTER, &new_value);
        
        new_value
    }
    
    pub fn get_count(env: Env) -> u32 {
        env.storage().instance()
            .get(&COUNTER)
            .unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::{Env, Address};

    #[test]
    fn test_increment() {
        let env = Env::default();
        let contract_id = env.register_contract(None, Counter);
        let client = CounterClient::new(&env, &contract_id);
        
        let user = Address::random(&env);
        
        assert_eq!(client.get_count(), 0);
        client.increment(&user);
        assert_eq!(client.get_count(), 1);
    }
}
```

### Step 2: Use Extension Features

As you type this code, notice:

1. **Auto-completion** when typing `#[contract]`
2. **Hover information** on `Env`, `Address`, `Symbol`
3. **Diagnostics** showing any missing imports or errors
4. **IntelliSense** for `env.storage()` methods

### Step 3: Build and Test

1. **Build the contract:**
   - Press `Ctrl+Shift+P` (or `Cmd+Shift+P`)
   - Type "Stellar: Build Stellar Contract"
   - Select the command

2. **Run tests:**
   - Press `Ctrl+Shift+P` (or `Cmd+Shift+P`)
   - Type "Stellar: Test Stellar Contract"
   - Select the command

## 🎯 Advanced Features

### Working with Test Contracts

The extension includes 8 example contracts in the `test-contracts/` directory:

1. **Access Control NFT** (`access_control_nft.rs`)
2. **Advanced Marketplace** (`advanced_marketplace.rs`)
3. **Allowlist Token** (`allowlist_token.rs`)
4. **Blocklist Token** (`blocklist_token.rs`)
5. **Multi-Extension Token** (`multi_extension_token.rs`)
6. **Ownable Counter** (`ownable_counter.rs`)
7. **Pausable Token** (`pausable_token.rs`)
8. **LSP Test Contract** (`lsp_test.rs`)

**To explore these contracts:**
1. Open any `.rs` file in the `test-contracts/` directory
2. Use the extension features to understand the code
3. Copy patterns to your own contracts

### Configuration Options

Access via VS Code Settings (`Ctrl+,` or `Cmd+,`):

```json
{
  "stellar.lsp.enable": true,
  "stellar.lsp.trace.server": "messages",
  "stellar.diagnostics.enable": true,
  "stellar.cli.path": "stellar",
  "stellar.build.target": "wasm32-unknown-unknown",
  "stellar.network.rpc": "https://soroban-testnet.stellar.org",
  "stellar.network.passphrase": "Test SDF Network ; September 2015"
}
```

## 🔍 Troubleshooting

### Common Issues and Solutions

1. **Extension not activating:**
   - Ensure you're working with `.rs` files
   - Check that VS Code version is 1.101.0+
   - Restart VS Code

2. **No completions showing:**
   - Verify the LSP server is running (check status bar)
   - Use command: "Stellar: Restart LSP Server"

3. **Build/test commands not working:**
   - Ensure Stellar CLI is installed and in PATH
   - Check `stellar.cli.path` setting

4. **Diagnostics not showing:**
   - Enable diagnostics in settings: `stellar.diagnostics.enable: true`
   - Check for syntax errors in your code

## 📚 Next Steps

1. **Read the Documentation:**
   - [Stellar Smart Contracts Guide](https://developers.stellar.org/docs/build/smart-contracts/)
   - [Soroban SDK Documentation](https://docs.rs/soroban-sdk/)

2. **Explore Examples:**
   - Study the included test contracts
   - Try the [Stellar Quest challenges](https://quest.stellar.org/)

3. **Join the Community:**
   - [Stellar Discord](https://discord.gg/stellar)
   - [Stellar Stack Overflow](https://stackoverflow.com/questions/tagged/stellar)
   - [GitHub Discussions](https://github.com/stellar/soroban-tools/discussions)

## 💡 Tips for Maximum Productivity

1. **Use Command Palette:** `Ctrl+Shift+P` for quick access to all Stellar commands
2. **Hover for Documentation:** Hover over any type or function for instant help
3. **Auto-save:** Enable auto-save for real-time diagnostics
4. **Split View:** Use split view to compare different contract implementations
5. **Integrated Terminal:** Use the integrated terminal for CLI operations

---

**Happy Coding with Stellar Smart Contracts!** 🚀

For more help, visit the [GitHub repository](https://github.com/sagpatil/Stellar-Smart-Contracts-LSP) or create an issue for support.

# Extension Features Demo

## 🎥 Live Example: Building a Token Contract

### Step 1: Create New File
```
File: src/token.rs
```

### Step 2: Type Contract Structure
```rust
#![no_std]
use soroban_sdk::{contract, contractimpl, Env, Address, Symbol};

#[contract]
pub struct Token;

#[contractimpl]
impl Token {
    // As you type this, the extension provides:
    // ✅ Auto-completion for contract attributes
    // ✅ Hover info on types
    // ✅ Real-time error checking
}
```

### Step 3: Add Functions with Extension Help

**Auto-completion in action:**
```rust
pub fn balance(env: Env, account: Address) -> i128 {
    env.storage().instance().get(&account).unwrap_or(0)
    //    ^^^^^^^ 
    // Type 'env.' and see completions:
    // - env.storage()
    // - env.current_contract_address()
    // - env.invoke_contract()
    // - env.random()
}
```

**Hover information example:**
```rust
pub fn transfer(env: Env, from: Address, to: Address, amount: i128) {
    from.require_auth();
    //   ^^^^^^^^^^^^
    // Hover shows: "Requires authentication from the address"
    
    // Extension suggests: Add balance checks
    // Extension warns: Consider overflow protection
}
```

### Step 4: Use CLI Integration

**Command Palette (`Ctrl+Shift+P`):**
```
> Stellar: Build Stellar Contract
  Running: stellar contract build
  ✅ Built target/wasm32-unknown-unknown/release/token.wasm

> Stellar: Test Stellar Contract  
  Running: stellar contract test
  ✅ Running 3 tests... OK
```

## 🔍 Extension Diagnostics in Action

### Error Detection
```rust
#[contract]  // ✅ Green checkmark - valid attribute
pub struct Token;

#[contractimpl]  // ✅ Green checkmark - valid attribute
impl Token {
    pub fn transfer(env: Env, amount: i128) {
        // ❌ Red underline: Missing authentication
        // 💡 Suggestion: Add user parameter and require_auth()
        
        env.storage().instance().set("balance", amount);
        // ❌ Red underline: Key should be Symbol, not &str
        // 💡 Suggestion: Use Symbol::new(&env, "balance")
    }
}
```

### Fixed Version with Extension Help
```rust
#[contract]
pub struct Token;

#[contractimpl]
impl Token {
    pub fn transfer(env: Env, from: Address, amount: i128) {
        from.require_auth();  // ✅ Added based on extension suggestion
        
        let key = Symbol::new(&env, "balance");  // ✅ Fixed type
        env.storage().instance().set(&key, &amount);  // ✅ All green
    }
}
```

## 📋 Available Completions

### Contract Attributes
- `#[contract]` → Full contract attribute
- `#[contractimpl]` → Implementation attribute  
- `#[contracttype]` → Data type attribute
- `#[contractclient]` → Client generation attribute

### Stellar Types
- `Env` → Contract environment
- `Address` → Account/contract address
- `Symbol` → Interned string
- `Bytes` → Byte array
- `Map<K, V>` → Key-value map
- `Vec<T>` → Dynamic array

### Environment Methods
- `env.storage()` → Storage operations
- `env.current_contract_address()` → Get current contract
- `env.invoke_contract()` → Call other contracts
- `env.random()` → Random number generation
- `env.events()` → Event publishing

### Storage Operations
- `.instance()` → Instance storage
- `.temporary()` → Temporary storage
- `.persistent()` → Persistent storage
- `.set()` → Store value
- `.get()` → Retrieve value
- `.has()` → Check existence

## 🎯 Real-World Usage Tips

### 1. Start Simple
```rust
// Begin with basic structure
#[contract]
pub struct MyContract;

#[contractimpl]
impl MyContract {
    pub fn init(env: Env) {
        // Extension helps with initialization patterns
    }
}
```

### 2. Use Extension Feedback
- **Green underlines**: Valid code
- **Yellow warnings**: Suggestions for improvement
- **Red errors**: Must fix before compilation
- **Blue info**: Additional information

### 3. Leverage Hover Documentation
- Hover over any type to see usage examples
- Hover over functions to see parameter requirements
- Hover over errors to see fix suggestions

### 4. Command Palette Workflow
```
1. Write code → Extension provides real-time feedback
2. Ctrl+Shift+P → "Stellar: Build" → Check compilation
3. Ctrl+Shift+P → "Stellar: Test" → Run tests
4. Fix issues → Repeat until green
5. Ctrl+Shift+P → "Stellar: Deploy" → Deploy to network
```

## 📊 Extension Status Indicators

Look for these in the VS Code status bar:
- `🟢 Stellar LSP: Running` → Extension working normally
- `🟡 Stellar LSP: Starting` → Extension initializing
- `🔴 Stellar LSP: Error` → Extension needs restart
- `📦 Building...` → Contract compilation in progress

## 🚀 Next Steps

1. **Try the examples** in `test-contracts/`
2. **Read hover documentation** for each type you use
3. **Use Command Palette** for all Stellar operations
4. **Check diagnostics** before building
5. **Explore advanced patterns** in the included contracts

---

**The extension turns VS Code into a powerful Stellar development environment!** ✨

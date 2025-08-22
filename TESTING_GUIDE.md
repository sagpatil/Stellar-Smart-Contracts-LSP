# Testing Guide: Stellar Smart Contracts LSP Extension

## 🚀 Fresh Package Ready for Testing!

**Package**: `stellar-contracts-lsp-0.0.1.vsix` (3.96 MB)  
**Created**: July 9, 2025  
**Status**: All fixes implemented and ready for testing

## 🔧 What's Been Fixed and Improved

### ✅ **Core LSP Features Fixed**
- **Document Symbols**: Fixed `textDocument/documentSymbol` error
- **TypeScript Bindings**: Fixed path handling with spaces in file paths
- **Build Command**: Fixed deprecated `--target` parameter
- **Auto-completion**: 20+ intelligent completions for Stellar SDK
- **Hover Information**: Rich documentation on hover
- **Real-time Diagnostics**: Comprehensive error checking

### ✅ **VS Code Commands Working**
- `Stellar: Build Stellar Contract` → `stellar contract build`
- `Stellar: Test Stellar Contract` → `stellar contract test`  
- `Stellar: Generate TypeScript Types` → Creates TypeScript bindings
- `Stellar: Deploy Stellar Contract` → Deploy with proper WASM handling
- `Stellar: Invoke Stellar Contract Function` → Call contract functions
- `Stellar: Restart LSP Server` → Restart if needed

## 🧪 Testing Instructions

### 1. **Install the Extension**
```bash
# Install the fresh package
code --install-extension stellar-contracts-lsp-0.0.1.vsix

# Or via VS Code GUI:
# Extensions → "..." menu → Install from VSIX → Select file
```

### 2. **Test Basic LSP Features**

**Open any contract file** (e.g., `access_control_nft.rs`):

**Auto-completion:**
- Type `#[con` → Should complete to `#[contract]`
- Type `env.` → Should show storage(), events(), etc.
- Type `Address::` → Should show static methods

**Hover Information:**
- Hover over `Env` → Should show documentation
- Hover over `Address` → Should show type info
- Hover over any function → Should show parameter details

**Document Symbols (NEW!):**
- Open Outline panel (View → Outline)
- Should see contract structure with functions
- Click on any symbol to jump to it
- Use `Ctrl+Shift+O` to search symbols

### 3. **Test VS Code Commands**

**Open Command Palette** (`Ctrl+Shift+P` or `Cmd+Shift+P`):

1. **Build Contract:**
   - Type: `Stellar: Build`
   - Should run `stellar contract build` (no more `--target` error!)

2. **Generate TypeScript Bindings:**
   - Type: `Stellar: Generate`
   - Should create `bindings/` directory with TypeScript files
   - No more path with spaces errors!

3. **Test Contract:**
   - Type: `Stellar: Test`
   - Should run contract tests

### 4. **Test Advanced Features**

**Diagnostics:**
- Try removing `#[contract]` attribute → Should show error
- Try missing authentication → Should show warning
- Try incorrect types → Should highlight issues

**Navigation:**
- Use `Ctrl+Click` on function names
- Use breadcrumb navigation
- Try "Go to Definition" features

## 📋 Test Contracts Included

The extension includes 8+ test contracts for testing features:

1. **access_control_nft.rs** - Role-based NFT contract
2. **allowlist_token.rs** - Token with allowlist functionality
3. **blocklist_token.rs** - Token with blocklist functionality
4. **pausable_token.rs** - Pausable token contract
5. **ownable_counter.rs** - Ownable counter with access control
6. **advanced_marketplace.rs** - NFT marketplace
7. **voting_contract.rs** - Governance voting contract
8. **hello_world.rs** - Simple greeting contract

## 🎯 Key Features to Test

### **1. Smart Completions**
```rust
#[contract]  // ← Type #[con and press Tab
pub struct MyContract;

#[contractimpl]  // ← Type #[contractimpl
impl MyContract {
    pub fn test(env: Env) {  // ← Type Env and see completion
        env.  // ← Type . and see methods
    }
}
```

### **2. Document Outline** (Fixed!)
- Should see hierarchical structure
- Contract types, structs, implementations
- Functions with proper icons
- Clickable navigation

### **3. TypeScript Bindings** (Fixed!)
- Build a contract first
- Use `Stellar: Generate TypeScript Types`
- Should create `bindings/` with TypeScript files
- No path errors with spaces!

### **4. Build Integration** (Fixed!)
- `Stellar: Build` now uses correct command
- No more "unexpected argument '--target'" error
- Shows build output in terminal

## 🚨 Known Test Contract Issues

Some test contracts have compilation issues (symbol conflicts), but **the extension features work correctly**. The LSP provides proper feedback about these issues through diagnostics.

## 📊 Status Indicators

Look for these in VS Code:
- **Status Bar**: `🟢 Stellar LSP: Running`
- **Outline Panel**: Shows contract structure
- **Problems Panel**: Shows diagnostics
- **Terminal**: Shows command output

## 🔍 Troubleshooting

If any feature doesn't work:
1. **Restart LSP**: Use `Stellar: Restart LSP Server`
2. **Check Output**: View "Stellar LSP" in Output panel
3. **Reload Window**: `Ctrl+Shift+P` → "Reload Window"

---

## 🎉 Ready for Full Testing!

The extension is now ready for comprehensive testing with all major issues fixed:
- ✅ Document symbols working
- ✅ TypeScript bindings generation working  
- ✅ Build commands working
- ✅ Comprehensive LSP features
- ✅ Rich documentation and examples

**Happy testing!** 🚀

# 🧪 Stellar VS Code Extension Command Palette Testing Guide

## Prerequisites
- VS Code is open with the Stellar LSP workspace
- Extension is compiled and running (npm run compile completed)
- Test contract built successfully with wasm32v1-none target
- WASM file available at: `target/wasm32v1-none/release/hello_world_standalone.wasm`

## Command Palette Tests (Cmd+Shift+P)

### 1. ✅ **Stellar: Build Stellar Contract**
**Test Steps:**
1. Open Command Palette: `Cmd+Shift+P`
2. Type: "Stellar: Build"
3. Select: "Stellar: Build Stellar Contract"

**Expected Result:**
- New terminal opens named "Stellar Build"
- Runs: `stellar contract build` command
- Should build the contract using wasm32v1-none target

**Status:** ✅ Ready to test

---

### 2. ✅ **Stellar: Test Stellar Contract**
**Test Steps:**
1. Open Command Palette: `Cmd+Shift+P`
2. Type: "Stellar: Test"
3. Select: "Stellar: Test Stellar Contract"

**Expected Result:**
- New terminal opens named "Stellar Test"
- Runs: `stellar contract test` command
- Should run contract tests

**Status:** ✅ Ready to test

---

### 3. ✅ **Stellar: Deploy Stellar Contract**
**Test Steps:**
1. Open Command Palette: `Cmd+Shift+P`
2. Type: "Stellar: Deploy"
3. Select: "Stellar: Deploy Stellar Contract"

**Expected Result:**
- Prompts for WASM file selection
- Shows available WASM files from target directory
- New terminal opens named "Stellar Deploy"
- Runs deployment command with RPC URL and network passphrase

**Status:** ✅ Ready to test (WASM file available)

---

### 4. ✅ **Stellar: Invoke Stellar Contract Function**
**Test Steps:**
1. Open Command Palette: `Cmd+Shift+P`  
2. Type: "Stellar: Invoke"
3. Select: "Stellar: Invoke Stellar Contract Function"

**Expected Result:**
- Prompts for Contract ID input
- Prompts for Function Name input  
- New terminal opens named "Stellar Invoke"
- Runs invoke command with specified parameters

**Status:** ✅ Ready to test

---

### 5. ✅ **Stellar: Generate TypeScript Types**
**Test Steps:**
1. Open Command Palette: `Cmd+Shift+P`
2. Type: "Stellar: Generate"  
3. Select: "Stellar: Generate TypeScript Types"

**Expected Result:**
- Automatically finds WASM files in target directory
- If multiple WASM files, shows selection dialog
- New terminal opens named "Stellar Generate Types"
- Runs: `stellar contract bindings typescript --wasm "[WASM_PATH]" --output-dir "[OUTPUT_DIR]" --overwrite`
- Creates TypeScript bindings in ./bindings directory

**Status:** ✅ Ready to test (WASM file available)

---

### 6. ✅ **Stellar: Restart Stellar LSP Server**
**Test Steps:**
1. Open Command Palette: `Cmd+Shift+P`
2. Type: "Stellar: Restart"
3. Select: "Stellar: Restart Stellar LSP Server"

**Expected Result:**
- Shows info message: "Stellar LSP server restarted"
- LSP server stops and restarts
- Language features continue working

**Status:** ✅ Ready to test

---

## LSP Features to Test in Rust Files

### In `lsp_test.rs` or any `.rs` file:

1. **Syntax Highlighting** ✅
   - Rust keywords should be highlighted
   - Soroban types like `Env`, `Address`, `Symbol` should be highlighted

2. **Code Completion** ✅
   - Type `env.` and see completions for `storage()`, `events()`, etc.
   - Type `Symbol::` and see `new()` completion

3. **Hover Information** ✅  
   - Hover over `Env` to see type information
   - Hover over `Address` to see documentation

4. **Diagnostics** ✅
   - Should see warnings for unused variables
   - Should see errors for syntax issues

5. **Document Symbols** ✅
   - Open document outline (Cmd+Shift+O)
   - Should see contract functions and structs

---

## Configuration Settings

The extension uses these settings (configurable in VS Code settings):

```json
{
  "stellar.lsp.enable": true,
  "stellar.lsp.trace.server": "off",
  "stellar.diagnostics.enable": true,
  "stellar.cli.path": "stellar",
  "stellar.network.rpc": "https://soroban-testnet.stellar.org",
  "stellar.network.passphrase": "Test SDF Network ; September 2015"
}
```

---

## Test Results Log

**Date:** 2025-07-09
**Extension Version:** 0.0.1
**VS Code Version:** 1.101.2
**Target:** wasm32v1-none ✅

### Command Tests:
- [ ] Stellar: Build Stellar Contract
- [ ] Stellar: Test Stellar Contract  
- [ ] Stellar: Deploy Stellar Contract
- [ ] Stellar: Invoke Stellar Contract Function
- [ ] Stellar: Generate TypeScript Types
- [ ] Stellar: Restart Stellar LSP Server

### LSP Feature Tests:
- [ ] Syntax Highlighting
- [ ] Code Completion  
- [ ] Hover Information
- [ ] Diagnostics
- [ ] Document Symbols

---

## Notes
- All commands use the `stellar` CLI tool
- WASM files are looked up in `**/target/**/*.wasm`
- TypeScript bindings are generated to `./bindings` directory
- Extension handles file paths with spaces correctly using quotes

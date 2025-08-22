# Sample Usage: Stellar Smart Contracts LSP Extension

## 🚀 Complete Workflow Example

### Step 1: Install and Setup
1. Install the `stellar-contracts-lsp-0.0.1.vsix` extension in VS Code
2. Open a Stellar contract project in VS Code
3. The extension activates automatically for `.rs` files

### Step 2: Write Your Contract

Create or open a contract file like `access_control_nft.rs`:

```rust
use soroban_sdk::{contract, contractimpl, contracttype, Env, Address, Symbol, Bytes};

#[contract]
pub struct AccessControlNFT;

#[contractimpl]
impl AccessControlNFT {
    pub fn initialize(env: Env, admin: Address, name: Symbol, symbol: Symbol) {
        // Contract logic here
    }
    
    pub fn mint(env: Env, to: Address, metadata: Bytes) -> u64 {
        // Minting logic here
        1
    }
}
```

### Step 3: Use Extension Features While Coding

**Auto-completion:**
- Type `#[con` → Extension suggests `#[contract]`
- Type `env.` → See all environment methods
- Type `Address::` → See Address static methods

**Hover Information:**
- Hover over `Env` → See documentation
- Hover over `Address` → See type information
- Hover over functions → See parameter details

**Real-time Diagnostics:**
- Missing `#[contract]` attribute → Red underline with suggestion
- Incorrect parameter types → Error highlighting
- Authentication missing → Warning with suggestion

### Step 4: Build Your Contract

**Using Extension Commands:**

1. **Open Command Palette:** `Ctrl+Shift+P` (or `Cmd+Shift+P` on Mac)

2. **Build Contract:**
   - Type: `Stellar: Build Stellar Contract`
   - Select the command
   - Extension runs: `stellar contract build`
   - Shows build output in terminal

3. **Test Contract:**
   - Type: `Stellar: Test Stellar Contract`
   - Select the command
   - Extension runs: `stellar contract test`
   - Shows test results

### Step 5: Generate TypeScript Bindings (Key Feature!)

**This is the command you used:**

1. **Open Command Palette:** `Ctrl+Shift+P` (or `Cmd+Shift+P`)

2. **Generate TypeScript Bindings:**
   - Type: `Stellar: Generate TypeScript Types`
   - Select the command
   - Extension automatically:
     - Finds your built WASM file
     - Runs `stellar contract bindings typescript`
     - Creates TypeScript bindings in your project

3. **What Gets Generated:**
   ```
   bindings/
   ├── package.json
   ├── tsconfig.json
   └── src/
       └── index.ts
   ```

4. **Generated TypeScript Interface:**
   ```typescript
   export interface Client {
     initialize: ({admin, name, symbol}: {
       admin: string,
       name: string, 
       symbol: string
     }) => Promise<AssembledTransaction<void>>
     
     mint: ({to, metadata}: {
       to: string,
       metadata: Buffer
     }) => Promise<AssembledTransaction<bigint>>
   }
   ```

### Step 6: Use in Your Frontend

**Install the generated bindings:**
```bash
cd bindings
npm install
npm run build
```

**Use in your TypeScript/JavaScript project:**
```typescript
import { Client } from './bindings/dist/index.js';
import { SorobanRpc } from '@stellar/stellar-sdk';

const rpc = new SorobanRpc.Server('https://soroban-testnet.stellar.org');
const contractId = 'CAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAD2KM';

const client = new Client({
  contractId,
  rpc,
  networkPassphrase: 'Test SDF Network ; September 2015'
});

// Call contract functions with full TypeScript support!
const result = await client.mint({
  to: 'GAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAABGXK',
  metadata: Buffer.from('NFT metadata')
});
```

## 🎯 Extension Commands Reference

| Command | Purpose | Shortcut |
|---------|---------|----------|
| `Stellar: Build Stellar Contract` | Compile contract to WASM | `Ctrl+Shift+P` → "Build" |
| `Stellar: Test Stellar Contract` | Run contract tests | `Ctrl+Shift+P` → "Test" |
| `Stellar: Generate TypeScript Types` | Create TS bindings | `Ctrl+Shift+P` → "Generate" |
| `Stellar: Deploy Stellar Contract` | Deploy to network | `Ctrl+Shift+P` → "Deploy" |
| `Stellar: Invoke Stellar Contract Function` | Call contract functions | `Ctrl+Shift+P` → "Invoke" |
| `Stellar: Restart LSP Server` | Fix LSP issues | `Ctrl+Shift+P` → "Restart" |

## 🔥 Pro Tips

### 1. **Automatic WASM Detection**
The extension automatically finds your built WASM files when generating TypeScript bindings. No need to specify paths manually!

### 2. **Integrated Workflow**
```
Write Contract → Build → Test → Generate Bindings → Deploy
     ↑                                                  ↓
     └──────── All from VS Code Command Palette ────────┘
```

### 3. **Real-time Feedback**
- **Status Bar**: Shows extension status
- **Problems Panel**: Shows diagnostics
- **Terminal**: Shows command output
- **Hover**: Shows instant documentation

### 4. **IntelliSense Everywhere**
The extension provides completions for:
- Contract attributes (`#[contract]`, `#[contractimpl]`)
- Stellar types (`Env`, `Address`, `Symbol`)
- Environment methods (`env.storage()`, `env.events()`)
- Storage operations (`.instance()`, `.set()`, `.get()`)

## 📋 Troubleshooting

### Extension Not Working?
1. **Check File Type**: Extension only activates for `.rs` files
2. **Restart LSP**: Use `Stellar: Restart LSP Server` command
3. **Check Output**: View "Stellar LSP" in Output panel

### TypeScript Generation Fails?
1. **Build First**: Ensure contract is built (`Stellar: Build`)
2. **Check WASM**: Verify WASM file exists in `target/` directory
3. **Stellar CLI**: Ensure Stellar CLI is installed and in PATH

### No Completions?
1. **File Extension**: Must be `.rs` file
2. **LSP Status**: Check status bar for LSP server status
3. **Settings**: Verify `stellar.lsp.enable` is `true`

## 🌟 Advanced Features

### Custom Settings
```json
{
  "stellar.lsp.enable": true,
  "stellar.lsp.trace.server": "messages",
  "stellar.diagnostics.enable": true,
  "stellar.cli.path": "stellar",
  "stellar.build.target": "wasm32v1-none"
}
```

### Multi-Contract Projects
The extension handles workspaces with multiple contracts:
- Detects all `.rs` files
- Provides completions across files
- Builds entire workspace
- Generates bindings for each contract

---

**The extension transforms VS Code into a complete Stellar development environment!** 🚀

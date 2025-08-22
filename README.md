# Stellar Smart Contracts LSP

A VS Code extension that provides Language Server Protocol (LSP) support for Stellar Smart Contracts development on the Stellar network.

## Features

This extension provides comprehensive language support for Stellar smart contracts written in Rust:

- **Syntax Highlighting**: Enhanced syntax highlighting for Stellar-specific constructs
- **Code Completion**: Intelligent code completion for Stellar types, attributes, and functions
- **Diagnostics**: Real-time error detection and validation for Stellar contracts
- **Hover Information**: Detailed documentation on hover for Stellar types and functions
- **Go to Definition**: Navigate to symbol definitions within your contract code
- **Symbol Search**: Find and navigate to symbols across your workspace

### Stellar-Specific Features

- Support for Stellar contract attributes (`#[contract]`, `#[contractimpl]`, `#[contracttype]`)
- Completion for Stellar types (`Env`, `Address`, `Symbol`, `Bytes`, `Map`, `Vec`)
- Validation for proper contract structure and implementation
- Hover documentation for Stellar environment and host functions

## Requirements

- Visual Studio Code 1.101.0 or higher
- Node.js for development (if building from source)

## Extension Settings

This extension contributes the following settings:

- `stellar.lsp.enable`: Enable/disable the Stellar LSP server (default: `true`)
- `stellar.lsp.trace.server`: Control LSP server communication tracing (`off`, `messages`, `verbose`)
- `stellar.diagnostics.enable`: Enable/disable diagnostic reporting (default: `true`)
- `stellar.cli.path`: Path to the Stellar CLI executable (default: `stellar`)
- `stellar.network.rpc`: RPC URL for Stellar network
- `stellar.network.passphrase`: Network passphrase for Stellar network

## Commands

- `Stellar: Restart LSP Server`: Restart the language server if it becomes unresponsive
- `Stellar: Build Stellar Contract`: Build the current Stellar contract using Stellar CLI
- `Stellar: Test Stellar Contract`: Run tests for the current Stellar contract
- `Stellar: Deploy Stellar Contract`: Deploy the contract to the Stellar network
- `Stellar: Invoke Stellar Contract Function`: Invoke a function on a deployed contract
- `Stellar: Generate TypeScript Types`: Generate TypeScript bindings for the contract

### TypeScript Bindings Generation

The extension includes a powerful feature to automatically generate TypeScript bindings for your contracts:

1. **Build your contract first**: Use `Stellar: Build Stellar Contract`
2. **Open Command Palette**: `Ctrl+Shift+P` (or `Cmd+Shift+P`)
3. **Run**: `Stellar: Generate TypeScript Types`
4. **Result**: TypeScript bindings created in `bindings/` directory

Example generated interface:

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

## Debugging Support

The extension provides comprehensive debugging capabilities for Stellar contracts using Code LLDB:

### Prerequisites
- **Code LLDB Extension**: Automatically installed as a dependency
- **Rust Analyzer**: Automatically installed as a dependency for enhanced code completion

### Debugging Features
1. **Launch Debugging**: Start a new debug session for your contract
2. **Attach Debugging**: Attach to an already running contract process
3. **Break on Panic**: Automatically break when contracts panic during execution
4. **Integrated Terminal**: Debug output appears in VS Code's integrated terminal

### How to Debug
1. **Build your contract first**: Use `Stellar: Build Stellar Contract`
2. **Start debugging**: Use `Stellar: Debug Stellar Contract` or press `F5`
3. **Set breakpoints**: Click in the gutter next to line numbers
4. **Use debug console**: Inspect variables and execute expressions

### Debug Configurations
The extension provides several debug configurations:
- **Debug Stellar Contract**: Debug with debug build
- **Debug Stellar Contract (Release)**: Debug with release build
- **Attach to Stellar Contract**: Attach to running process

## Rust Analyzer Integration

Enhanced code completion and analysis through Rust Analyzer integration:

### Features
- **Intelligent Code Completion**: Context-aware suggestions for Rust syntax
- **Error Detection**: Real-time error highlighting and diagnostics
- **Go to Definition**: Navigate to function and type definitions
- **Find References**: Locate all usages of functions and types
- **Code Actions**: Automatic fixes and refactoring suggestions

### Configuration
Enable/disable Rust Analyzer integration in VS Code settings:
```json
{
  "stellar.rustAnalyzer.enable": true
}
```

## Stellar CLI Integration

This extension integrates with the Stellar CLI to provide seamless development experience:

- **Build**: Compile contracts to WASM with `stellar contract build`
- **Test**: Run contract tests with `stellar contract test`
- **Deploy**: Deploy contracts to Stellar networks
- **Invoke**: Call contract functions from VS Code
- **Generate Types**: Create TypeScript bindings for frontend integration

### CLI Configuration

Configure the Stellar CLI integration through VS Code settings:

- `stellar.cli.path`: Path to the Stellar CLI executable (default: `stellar`)
- `stellar.network.rpc`: RPC URL for Stellar network
- `stellar.network.passphrase`: Network passphrase for Stellar network

## Getting Started

### Quick Start
New to the extension? Check out our [5-minute Quick Start Guide](QUICKSTART.md) to get up and running immediately.

### Complete Usage Guide
For comprehensive documentation, see the [complete Usage Guide](USAGE_GUIDE.md) which covers:
- Step-by-step installation
- Feature walkthrough with examples
- Advanced usage patterns
- Troubleshooting tips

### Live Demo
Want to see the extension in action? Check out the [Demo Guide](DEMO.md) with real-world examples and screenshots.

## Development

To set up the development environment:

1. Clone the repository
2. Install dependencies: `npm install`
3. Install server dependencies: `cd server && npm install`
4. Compile the project: `npm run compile`
5. Press `F5` to run the extension in a new Extension Development Host window

## Building

- `npm run compile`: Compile the extension
- `npm run watch`: Watch for changes and recompile automatically
- `npm run package`: Package the extension for distribution

## Known Issues

- This is an initial implementation focusing on basic LSP features
- Advanced Stellar-specific analysis features are in development
- Some edge cases in contract validation may not be caught

## Release Notes

### 0.0.1

Initial release of Stellar Smart Contracts LSP:
- Basic syntax highlighting for Stellar constructs
- Code completion for Stellar types and attributes
- Enhanced diagnostic validation with comprehensive error checking
- Hover information for common Stellar types and functions
- Stellar CLI integration for build, test, deploy, and invoke operations
- TypeScript bindings generation support
- Comprehensive code completions including storage methods and environment functions
- Advanced contract validation and best practices suggestions

### Enhanced Features in v0.0.1
- **Extended Completions**: 20+ intelligent code completions for Stellar SDK
- **Advanced Diagnostics**: Comprehensive validation including:
  - Missing attribute detection
  - Authentication requirement checks
  - Storage usage validation
  - Best practices suggestions
- **CLI Integration**: Full Stellar CLI integration with VS Code commands
- **Rich Hover Information**: Detailed documentation for all Stellar types and functions
- **Test Contracts**: Example contracts for testing and development

## Contributing

Contributions are welcome! This extension is part of the Stellar Development Foundation's efforts to improve the developer experience for Stellar smart contracts.

---

**Stellar Development Foundation** - Building the future of decentralized finance on Stellar.


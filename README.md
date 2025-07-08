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

## Commands

- `Stellar: Restart LSP Server`: Restart the language server if it becomes unresponsive

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
- Simple diagnostic validation
- Hover information for common Stellar types

## Contributing

Contributions are welcome! This extension is part of the Stellar Development Foundation's efforts to improve the developer experience for Stellar smart contracts.

## License

This project is licensed under the MIT License.

---

**Stellar Development Foundation** - Building the future of decentralized finance on Stellar.

## Following extension guidelines

Ensure that you've read through the extensions guidelines and follow the best practices for creating your extension.

* [Extension Guidelines](https://code.visualstudio.com/api/references/extension-guidelines)

## Working with Markdown

You can author your README using Visual Studio Code. Here are some useful editor keyboard shortcuts:

* Split the editor (`Cmd+\` on macOS or `Ctrl+\` on Windows and Linux).
* Toggle preview (`Shift+Cmd+V` on macOS or `Shift+Ctrl+V` on Windows and Linux).
* Press `Ctrl+Space` (Windows, Linux, macOS) to see a list of Markdown snippets.

## For more information

* [Visual Studio Code's Markdown Support](http://code.visualstudio.com/docs/languages/markdown)
* [Markdown Syntax Reference](https://help.github.com/articles/markdown-basics/)

**Enjoy!**

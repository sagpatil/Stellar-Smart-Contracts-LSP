# Stellar Smart Contracts LSP - Deployment Guide

## 📦 Package Information

- **Extension Name**: Stellar Smart Contracts LSP
- **Version**: 0.0.1
- **Package File**: `stellar-contracts-lsp-0.0.1.vsix`
- **Package Size**: 120KB
- **Publisher**: stellar-development-foundation

## 🚀 Installation Methods

### Method 1: Install from VSIX file

1. **From Command Line:**
   ```bash
   code --install-extension stellar-contracts-lsp-0.0.1.vsix
   ```

2. **From VS Code UI:**
   - Open VS Code
   - Press `Ctrl+Shift+P` (or `Cmd+Shift+P` on macOS)
   - Type "Extensions: Install from VSIX..."
   - Select the `stellar-contracts-lsp-0.0.1.vsix` file

### Method 2: Publish to VS Code Marketplace

1. **Prerequisites:**
   ```bash
   npm install -g @vscode/vsce
   ```

2. **Create Publisher Account:**
   - Visit [Visual Studio Marketplace](https://marketplace.visualstudio.com/manage)
   - Create a publisher account

3. **Login and Publish:**
   ```bash
   vsce login <publisher-name>
   vsce publish
   ```

## 🛠 Building from Source

If you need to rebuild the extension:

1. **Clone the repository:**
   ```bash
   git clone <repository-url>
   cd stellar-contracts-lsp
   ```

2. **Install dependencies:**
   ```bash
   npm install
   cd server && npm install && cd ..
   ```

3. **Build the extension:**
   ```bash
   npm run package
   ```

4. **Package for deployment:**
   ```bash
   vsce package
   ```

## 📋 Package Contents

The packaged extension includes:

- **Core Extension**: `dist/extension.js` (350KB)
- **Language Server**: `server/server.js` (50KB)
- **Language Configuration**: `language-configuration.json`
- **Syntax Highlighting**: `syntaxes/rust.tmLanguage.json`
- **Test Contracts**: Sample Stellar contracts for testing
- **Documentation**: README, CHANGELOG, QUICKSTART guide

## 🔧 System Requirements

- **VS Code**: Version 1.101.0 or higher
- **Dependencies**: 
  - `rust-lang.rust-analyzer` (automatically installed)
  - `vadimcn.vscode-lldb` (automatically installed)

## ✅ Verification

After installation, verify the extension is working:

1. Open a `.rs` file containing Stellar contract code
2. Check that syntax highlighting is active
3. Test autocompletion with Stellar-specific types (`Env`, `Address`, etc.)
4. Verify hover information for Stellar functions
5. Test go-to-definition functionality

## 🐛 Troubleshooting

### Extension not loading:
- Check VS Code version compatibility
- Ensure all dependencies are installed
- Restart VS Code after installation

### Language features not working:
- Verify the file has `.rs` extension
- Check that the file contains Stellar contract code
- Open VS Code Developer Tools (`Help > Toggle Developer Tools`) for error messages

### Server connection issues:
- Check the Output panel (`View > Output > Stellar LSP`)
- Restart the language server: `Ctrl+Shift+P` > "Restart Language Server"

## 📝 Version History

- **v0.0.1**: Initial release with basic LSP features
  - Syntax highlighting for Stellar Rust
  - Code completion for Stellar types and functions
  - Hover information for Stellar APIs
  - Go-to-definition support
  - Diagnostic error reporting
  - OpenZeppelin Stellar integration

## 🔄 Updates

To update the extension:

1. **From Marketplace**: Updates will be automatic
2. **From VSIX**: Reinstall with the new VSIX file using the same installation method

## 📞 Support

For issues and support:
- GitHub Issues: [Repository Issues](https://github.com/stellar/stellar-contracts-lsp/issues)
- Documentation: See included README.md and guides
- Community: Stellar Developer Discord
#!/bin/bash

# Stellar Smart Contracts LSP - Package Verification Script

echo "🔍 Verifying Stellar Smart Contracts LSP Package..."
echo "=================================================="

# Check if VSIX file exists
if [ ! -f "stellar-contracts-lsp-0.0.1.vsix" ]; then
    echo "❌ Package file not found: stellar-contracts-lsp-0.0.1.vsix"
    exit 1
fi

# Get package size
SIZE=$(ls -lh stellar-contracts-lsp-0.0.1.vsix | awk '{print $5}')
echo "📦 Package size: $SIZE"

# Check package contents
echo ""
echo "📋 Package contents:"
if command -v vsce &> /dev/null; then
    vsce ls --tree stellar-contracts-lsp-0.0.1.vsix
else
    echo "⚠️  vsce not found. Install with: npm install -g @vscode/vsce"
fi

echo ""
echo "🧪 Testing installation..."

# Test installation (only if VS Code is available)
if command -v code &> /dev/null; then
    echo "Installing extension..."
    code --install-extension stellar-contracts-lsp-0.0.1.vsix --force
    
    if [ $? -eq 0 ]; then
        echo "✅ Extension installed successfully!"
        echo ""
        echo "🎯 Next steps:"
        echo "1. Open VS Code"
        echo "2. Open a .rs file with Stellar contract code"
        echo "3. Test features:"
        echo "   - Syntax highlighting"
        echo "   - Code completion (try typing 'Env', 'Address')"
        echo "   - Hover information"
        echo "   - Go to definition (Ctrl+Click)"
        echo ""
        echo "🔧 To uninstall: code --uninstall-extension stellar-development-foundation.stellar-contracts-lsp"
    else
        echo "❌ Installation failed"
        exit 1
    fi
else
    echo "⚠️  VS Code CLI not available. Install manually:"
    echo "   code --install-extension stellar-contracts-lsp-0.0.1.vsix"
fi

echo ""
echo "📁 Package ready for deployment!"
echo "🚀 See DEPLOYMENT.md for distribution options"
#!/bin/bash

# Create a simple contract for TypeScript bindings demo
# This script creates a new Stellar contract project and generates TypeScript bindings

echo "🚀 Creating Stellar Contract TypeScript Bindings Demo"
echo

# Create a temporary directory for the demo
DEMO_DIR="/tmp/stellar-ts-demo"
rm -rf "$DEMO_DIR"
mkdir -p "$DEMO_DIR"
cd "$DEMO_DIR"

echo "📁 Creating new Stellar contract project..."
stellar contract init greeting-contract
cd greeting-contract

echo "✨ Creating a simple greeting contract..."

# Create a simple contract
cat > src/lib.rs << 'EOF'
#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, Env, Symbol, String};

#[contract]
pub struct GreetingContract;

#[contractimpl]
impl GreetingContract {
    /// Returns a greeting message
    pub fn hello(env: Env, name: String) -> String {
        String::from_str(&env, &format!("Hello, {}!", name.to_string()))
    }
    
    /// Get a simple symbol
    pub fn get_symbol(env: Env) -> Symbol {
        symbol_short!("GREET")
    }
    
    /// Add two numbers
    pub fn add(env: Env, a: u32, b: u32) -> u32 {
        a + b
    }
}
EOF

echo "🔨 Building the contract..."
stellar contract build

if [ $? -eq 0 ]; then
    echo "✅ Contract built successfully!"
    
    WASM_PATH="target/wasm32-unknown-unknown/release/greeting_contract.wasm"
    
    if [ -f "$WASM_PATH" ]; then
        echo "📦 WASM file found at: $WASM_PATH"
        
        echo "🔧 Generating TypeScript bindings..."
        mkdir -p bindings
        
        stellar contract bindings typescript \
            --wasm "$WASM_PATH" \
            --output-dir bindings \
            --overwrite
        
        if [ $? -eq 0 ]; then
            echo "✅ TypeScript bindings generated successfully!"
            echo
            echo "📋 Generated files:"
            find bindings -name "*.ts" -exec echo "  - {}" \;
            echo
            echo "📄 Sample TypeScript binding content:"
            echo "----------------------------------------"
            head -20 bindings/dist/index.d.ts 2>/dev/null || head -20 bindings/*.ts 2>/dev/null || echo "No .ts files found"
            echo "----------------------------------------"
        else
            echo "❌ Failed to generate TypeScript bindings"
        fi
    else
        echo "❌ WASM file not found at expected path: $WASM_PATH"
        echo "Looking for WASM files:"
        find . -name "*.wasm" -type f
    fi
else
    echo "❌ Contract build failed"
fi

echo
echo "📁 Demo files created in: $DEMO_DIR/greeting-contract"
echo "🧹 To clean up: rm -rf $DEMO_DIR"

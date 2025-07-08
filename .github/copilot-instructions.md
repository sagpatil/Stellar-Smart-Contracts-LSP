# Copilot Instructions for Stellar Smart Contracts LSP

<!-- Use this file to provide workspace-specific custom instructions to Copilot. For more details, visit https://code.visualstudio.com/docs/copilot/copilot-customization#_use-a-githubcopilotinstructionsmd-file -->

This is a VS Code extension project that implements a Language Server Protocol (LSP) for Stellar Smart Contracts development. Please use the get_vscode_api with a query as input to fetch the latest VS Code API references.

## Project Context

This extension provides language server features for Stellar smart contracts, including:
- Syntax highlighting for Stellar Rust code
- Code completion and IntelliSense
- Diagnostic error reporting
- Hover information
- Go to definition
- Symbol search and references

## Key Technologies

- **Language Server Protocol (LSP)**: For language features
- **VS Code Extension API**: For editor integration
- **TypeScript**: Primary development language
- **Rust**: Target language for Stellar smart contracts
- **Stellar SDK**: For Stellar-specific functionality

## Development Guidelines

1. Follow VS Code extension best practices
2. Use the Language Server Protocol for all language features
3. Implement proper error handling and diagnostics
4. Ensure good performance for large codebases
5. Follow Stellar naming conventions and patterns
6. Provide comprehensive hover information for Stellar-specific types and functions

## Stellar-Specific Features

When implementing features, consider:
- Stellar contract structure and attributes
- Stellar types (Symbol, Address, Vec, Map, etc.)
- Stellar host functions and environment
- Contract deployment and testing patterns
- Stellar network integration

## Code Style

- Use TypeScript with strict typing
- Follow ESLint configuration
- Use meaningful variable and function names
- Add comprehensive JSDoc comments
- Implement proper error handling

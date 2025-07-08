# Change Log

All notable changes to the "stellar-contracts-lsp" extension will be documented in this file.

Check [Keep a Changelog](http://keepachangelog.com/) for recommendations on how to structure this file.

## [0.0.1] - 2025-07-08

### Added
- Initial release of Stellar Smart Contracts LSP extension
- Language Server Protocol support for Stellar smart contracts
- Syntax highlighting for Stellar-specific constructs
- Code completion for Stellar types and attributes
- Diagnostic validation for contract structure
- Hover information for Stellar types and functions
- Restart LSP server command

### Features
- Support for Stellar contract attributes (`#[contract]`, `#[contractimpl]`, `#[contracttype]`)
- Completion for Stellar types (`Env`, `Address`, `Symbol`, `Bytes`, `Map`, `Vec`)
- Real-time diagnostics for contract validation
- Configurable LSP server settings
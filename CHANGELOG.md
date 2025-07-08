# Change Log

All notable changes to the "stellar-contracts-lsp" extension will be documented in this file.

## [1.1.0] - 2025-01-08

### Added

#### Advanced Test Contracts
- **Allowlist Token** (`allowlist_token.rs`): OpenZeppelin-inspired token with allowlist functionality
- **Blocklist Token** (`blocklist_token.rs`): Token with blocklist capabilities for compliance
- **Pausable Token** (`pausable_token.rs`): Token with emergency stop mechanism
- **Ownable Counter** (`ownable_counter.rs`): Simple counter demonstrating ownership patterns
- **Access Control NFT** (`access_control_nft.rs`): NFT with role-based access control
- **Multi-Extension Token** (`multi_extension_token.rs`): Complex token combining multiple patterns
- **Advanced Marketplace** (`advanced_marketplace.rs`): Sophisticated marketplace with bidding, fees, and statistics

#### Enhanced LSP Features
- **OpenZeppelin Pattern Support**: Added 38+ new completions for OpenZeppelin patterns
- **Advanced Access Control**: Support for `#[only_admin]`, `#[only_owner]`, `#[only_role]`, `#[has_role]` attributes
- **Pausable Contract Support**: `#[when_not_paused]` and `#[when_paused]` macros
- **Role-Based Access Control**: Complete RBAC pattern support with hierarchical roles
- **Token Extension Patterns**: AllowList, BlockList, Burnable, and other token extensions

#### New Completions
- OpenZeppelin access control macros and imports
- Pausable contract patterns and functions
- Ownable contract patterns and utilities
- Fungible and non-fungible token extensions
- Default implementation patterns (`#[default_impl]`)
- Advanced storage and authentication patterns

#### Enhanced Hover Information
- **OpenZeppelin Documentation**: Comprehensive hover docs for all OpenZeppelin patterns
- **Access Control Info**: Detailed explanations of role-based access control
- **Token Extension Docs**: Information about allowlist, blocklist, and other extensions
- **Pausable Pattern Info**: Emergency stop mechanism documentation
- **Best Practices**: Security recommendations and usage guidelines

#### Improved Diagnostics
- **Pattern Validation**: Better detection of OpenZeppelin pattern usage
- **Security Warnings**: Enhanced warnings for missing authentication
- **Access Control Checks**: Validation of role and ownership patterns
- **Storage Usage**: Improved storage type validation
- **Error Handling**: Better detection of error handling patterns

### Enhanced
- **Code Completions**: Expanded from 20 to 58+ completion items
- **Hover Provider**: Added support for 15+ new OpenZeppelin concepts
- **Diagnostic Engine**: Enhanced with pattern-specific validations
- **Test Coverage**: Added 7 new complex test contracts

### Technical Improvements
- **LSP Server**: Enhanced TypeScript server with advanced pattern recognition
- **Syntax Highlighting**: Improved support for OpenZeppelin attributes and macros
- **Documentation**: Comprehensive inline documentation for all patterns
- **Code Snippets**: Parameterized snippets for rapid development

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
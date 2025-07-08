#![no_std]

/// Test contracts for the Stellar Smart Contracts LSP extension
/// 
/// This module contains various example contracts to test and demonstrate
/// the LSP features including syntax highlighting, completions, diagnostics,
/// and hover information.

// Basic contracts
mod hello_world;
mod token_contract;
mod voting_contract;

// Advanced contracts based on OpenZeppelin patterns
mod allowlist_token;
mod blocklist_token;
mod pausable_token;
mod ownable_counter;
mod access_control_nft;
mod multi_extension_token;
mod advanced_marketplace;

// LSP testing contract
mod lsp_test;

// Re-export specific contract structs and clients to avoid ambiguous glob re-exports
pub use hello_world::HelloContract;
pub use token_contract::{TokenContract, TokenMetadata};
pub use voting_contract::VotingContract;

// Advanced contracts
pub use allowlist_token::{AllowlistToken, AllowlistTokenClient, AllowlistEvent};
pub use blocklist_token::{BlocklistToken, BlocklistTokenClient, BlocklistEvent};
pub use pausable_token::{PausableToken, PausableTokenClient, PausableEvent};
pub use ownable_counter::{OwnableCounter, OwnableCounterClient, OwnableEvent};

// Access Control NFT, Multi-Extension Token, and Advanced Marketplace
pub use access_control_nft::{AccessControlNFT, AccessControlNFTClient, AccessControlEvent};
pub use multi_extension_token::{MultiExtensionToken, MultiExtensionTokenClient};
pub use advanced_marketplace::{AdvancedMarketplace, AdvancedMarketplaceClient, Listing, ListingStatus, MarketplaceEvent};

// LSP Test contract
pub use lsp_test::{LspTest, TestEvent};

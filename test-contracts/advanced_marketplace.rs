use soroban_sdk::{
    contract, contractimpl, contracttype, Address, Env, Vec, 
    symbol_short, token, contracterror
};

/// Status of a marketplace listing
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ListingStatus {
    Active,
    Sold,
    Cancelled,
    Expired,
}

/// A marketplace listing
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Listing {
    pub id: u64,
    pub seller: Address,
    pub token_contract: Address,
    pub token_id: u64,
    pub price: i128,
    pub currency: Address,
    pub status: ListingStatus,
    pub created_at: u64,
    pub expires_at: u64,
}

/// Marketplace events
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MarketplaceEvent {
    ListingCreated(u64, Address, Address, u64, i128), // listing_id, seller, token_contract, token_id, price
    ListingCancelled(u64, Address), // listing_id, seller
    ListingPurchased(u64, Address, Address, i128), // listing_id, seller, buyer, price
    ListingExpired(u64), // listing_id
    OwnershipTransferred(Address, Address), // previous_owner, new_owner
}

/// Marketplace error types
#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum MarketplaceError {
    NotAuthorized = 1,
    ListingNotFound = 2,
    ListingNotActive = 3,
    InsufficientFunds = 4,
    InvalidPrice = 5,
    TokenTransferFailed = 6,
    ListingExpired = 7,
    AlreadyInitialized = 8,
    NotInitialized = 9,
}

/// Storage keys for the marketplace
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DataKey {
    Owner,
    NextListingId,
    Listing(u64),
    UserListings(Address),
    MarketplaceFee,
    TreasuryAddress,
    Initialized,
}

#[contract]
pub struct AdvancedMarketplace;

#[contractimpl]
impl AdvancedMarketplace {
    /// Initialize the marketplace
    pub fn initialize(
        env: Env,
        owner: Address,
        treasury: Address,
        marketplace_fee: u32, // Fee in basis points (100 = 1%)
    ) -> Result<(), MarketplaceError> {
        if env.storage().instance().has(&DataKey::Initialized) {
            return Err(MarketplaceError::AlreadyInitialized);
        }

        owner.require_auth();
        
        env.storage().instance().set(&DataKey::Owner, &owner);
        env.storage().instance().set(&DataKey::TreasuryAddress, &treasury);
        env.storage().instance().set(&DataKey::MarketplaceFee, &marketplace_fee);
        env.storage().instance().set(&DataKey::NextListingId, &1u64);
        env.storage().instance().set(&DataKey::Initialized, &true);

        Ok(())
    }

    /// Create a new listing
    pub fn create_listing(
        env: Env,
        seller: Address,
        token_contract: Address,
        token_id: u64,
        price: i128,
        currency: Address,
        duration: u64, // Duration in seconds
    ) -> Result<u64, MarketplaceError> {
        Self::require_initialized(&env)?;
        seller.require_auth();

        if price <= 0 {
            return Err(MarketplaceError::InvalidPrice);
        }

        let listing_id = env.storage().instance().get(&DataKey::NextListingId).unwrap_or(1u64);
        
        let current_time = env.ledger().timestamp();
        let expires_at = current_time + duration;

        let listing = Listing {
            id: listing_id,
            seller: seller.clone(),
            token_contract: token_contract.clone(),
            token_id,
            price,
            currency: currency.clone(),
            status: ListingStatus::Active,
            created_at: current_time,
            expires_at,
        };

        // Store the listing
        env.storage().persistent().set(&DataKey::Listing(listing_id), &listing);
        
        // Update user listings
        let mut user_listings: Vec<u64> = env.storage()
            .persistent()
            .get(&DataKey::UserListings(seller.clone()))
            .unwrap_or(Vec::new(&env));
        user_listings.push_back(listing_id);
        env.storage().persistent().set(&DataKey::UserListings(seller.clone()), &user_listings);

        // Update next listing ID
        env.storage().instance().set(&DataKey::NextListingId, &(listing_id + 1));

        // Emit event
        env.events().publish(
            (symbol_short!("listing"), symbol_short!("created")),
            MarketplaceEvent::ListingCreated(
                listing_id,
                seller,
                token_contract,
                token_id,
                price,
            ),
        );

        Ok(listing_id)
    }

    /// Purchase a listing
    pub fn purchase_listing(
        env: Env,
        buyer: Address,
        listing_id: u64,
    ) -> Result<(), MarketplaceError> {
        Self::require_initialized(&env)?;
        buyer.require_auth();

        let mut listing: Listing = env.storage()
            .persistent()
            .get(&DataKey::Listing(listing_id))
            .ok_or(MarketplaceError::ListingNotFound)?;

        if listing.status != ListingStatus::Active {
            return Err(MarketplaceError::ListingNotActive);
        }

        let current_time = env.ledger().timestamp();
        if current_time > listing.expires_at {
            listing.status = ListingStatus::Expired;
            env.storage().persistent().set(&DataKey::Listing(listing_id), &listing);
            return Err(MarketplaceError::ListingExpired);
        }

        // Calculate fees
        let marketplace_fee: u32 = env.storage().instance().get(&DataKey::MarketplaceFee).unwrap_or(250); // 2.5% default
        let fee_amount = (listing.price * marketplace_fee as i128) / 10000;
        let seller_amount = listing.price - fee_amount;

        // Transfer payment to seller
        let currency_client = token::Client::new(&env, &listing.currency);
        currency_client.transfer(&buyer, &listing.seller, &seller_amount);

        // Transfer fee to treasury
        if fee_amount > 0 {
            let treasury: Address = env.storage().instance().get(&DataKey::TreasuryAddress).unwrap();
            currency_client.transfer(&buyer, &treasury, &fee_amount);
        }

        // Transfer NFT to buyer
        let token_client = token::Client::new(&env, &listing.token_contract);
        token_client.transfer(&listing.seller, &buyer, &(listing.token_id as i128));

        // Update listing status
        listing.status = ListingStatus::Sold;
        env.storage().persistent().set(&DataKey::Listing(listing_id), &listing);

        // Emit event
        env.events().publish(
            (symbol_short!("listing"), symbol_short!("purchased")),
            MarketplaceEvent::ListingPurchased(
                listing_id,
                listing.seller.clone(),
                buyer.clone(),
                listing.price,
            ),
        );

        Ok(())
    }

    /// Cancel a listing
    pub fn cancel_listing(
        env: Env,
        seller: Address,
        listing_id: u64,
    ) -> Result<(), MarketplaceError> {
        Self::require_initialized(&env)?;
        seller.require_auth();

        let mut listing: Listing = env.storage()
            .persistent()
            .get(&DataKey::Listing(listing_id))
            .ok_or(MarketplaceError::ListingNotFound)?;

        if listing.seller != seller {
            return Err(MarketplaceError::NotAuthorized);
        }

        if listing.status != ListingStatus::Active {
            return Err(MarketplaceError::ListingNotActive);
        }

        listing.status = ListingStatus::Cancelled;
        env.storage().persistent().set(&DataKey::Listing(listing_id), &listing);

        // Emit event
        env.events().publish(
            (symbol_short!("listing"), symbol_short!("cancelled")),
            MarketplaceEvent::ListingCancelled(
                listing_id,
                seller,
            ),
        );

        Ok(())
    }

    /// Get a listing by ID
    pub fn get_listing(env: Env, listing_id: u64) -> Option<Listing> {
        env.storage().persistent().get(&DataKey::Listing(listing_id))
    }

    /// Get user listings
    pub fn get_user_listings(env: Env, user: Address) -> Vec<u64> {
        env.storage()
            .persistent()
            .get(&DataKey::UserListings(user))
            .unwrap_or(Vec::new(&env))
    }

    /// Update marketplace fee (owner only)
    pub fn update_marketplace_fee(
        env: Env,
        new_fee: u32,
    ) -> Result<(), MarketplaceError> {
        Self::require_initialized(&env)?;
        let owner: Address = env.storage().instance().get(&DataKey::Owner).unwrap();
        owner.require_auth();

        env.storage().instance().set(&DataKey::MarketplaceFee, &new_fee);
        Ok(())
    }

    /// Update treasury address (owner only)
    pub fn update_treasury(
        env: Env,
        new_treasury: Address,
    ) -> Result<(), MarketplaceError> {
        Self::require_initialized(&env)?;
        let owner: Address = env.storage().instance().get(&DataKey::Owner).unwrap();
        owner.require_auth();

        env.storage().instance().set(&DataKey::TreasuryAddress, &new_treasury);
        Ok(())
    }

    /// Transfer ownership
    pub fn transfer_ownership(
        env: Env,
        new_owner: Address,
    ) -> Result<(), MarketplaceError> {
        Self::require_initialized(&env)?;
        let current_owner: Address = env.storage().instance().get(&DataKey::Owner).unwrap();
        current_owner.require_auth();

        env.storage().instance().set(&DataKey::Owner, &new_owner);

        // Emit event
        env.events().publish(
            (symbol_short!("owner"), symbol_short!("transfer")),
            MarketplaceEvent::OwnershipTransferred(
                current_owner,
                new_owner,
            ),
        );

        Ok(())
    }

    /// Get marketplace fee
    pub fn get_marketplace_fee(env: Env) -> u32 {
        env.storage().instance().get(&DataKey::MarketplaceFee).unwrap_or(250)
    }

    /// Get treasury address
    pub fn get_treasury(env: Env) -> Option<Address> {
        env.storage().instance().get(&DataKey::TreasuryAddress)
    }

    /// Get owner address
    pub fn get_owner(env: Env) -> Option<Address> {
        env.storage().instance().get(&DataKey::Owner)
    }

    /// Check if marketplace is initialized
    pub fn is_initialized(env: Env) -> bool {
        env.storage().instance().get(&DataKey::Initialized).unwrap_or(false)
    }

    /// Private helper to check if marketplace is initialized
    fn require_initialized(env: &Env) -> Result<(), MarketplaceError> {
        if !env.storage().instance().get(&DataKey::Initialized).unwrap_or(false) {
            return Err(MarketplaceError::NotInitialized);
        }
        Ok(())
    }
}



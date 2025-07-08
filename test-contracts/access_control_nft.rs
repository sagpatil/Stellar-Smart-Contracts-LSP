// Simplified access control NFT contract demonstrating role-based access control
// Uses basic Soroban SDK features without OpenZeppelin dependencies
use soroban_sdk::{contract, contractimpl, contracttype, Env, Address, Symbol, Bytes};

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Admin,
    Roles(Address, Symbol), // (address, role)
    TokenOwner(u64),        // token_id -> owner
    TokenMetadata(u64),     // token_id -> metadata
    NextTokenId,
    TokenName,
    TokenSymbol,
}

#[contracttype]
#[derive(Clone)]
pub enum AccessControlEvent {
    RoleGranted(Address, Symbol),
    RoleRevoked(Address, Symbol),
    NFTMinted(Address, u64),
    NFTBurned(u64),
}

#[contract]
pub struct AccessControlNFT;

#[contractimpl]
impl AccessControlNFT {
    /// Initialize the NFT contract with role-based access control
    pub fn initialize(env: Env, admin: Address, name: Symbol, symbol: Symbol) {
        if env.storage().instance().has(&DataKey::Admin) {
            panic!("Already initialized");
        }
        
        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().instance().set(&DataKey::TokenName, &name);
        env.storage().instance().set(&DataKey::TokenSymbol, &symbol);
        env.storage().instance().set(&DataKey::NextTokenId, &1u64);
        
        // Grant admin role to the deployer
        let admin_role = Symbol::new(&env, "admin");
        env.storage().instance().set(&DataKey::Roles(admin.clone(), admin_role.clone()), &true);
        
        env.events().publish((Symbol::new(&env, "role_granted"),), (admin, admin_role));
    }
    
    /// Check if address has a specific role
    pub fn has_role(env: Env, account: Address, role: Symbol) -> bool {
        env.storage().instance().get(&DataKey::Roles(account, role)).unwrap_or(false)
    }
    
    /// Grant a role to an address (admin only)
    pub fn grant_role(env: Env, account: Address, role: Symbol) {
        let admin: Address = env.storage().instance().get(&DataKey::Admin).unwrap();
        admin.require_auth();
        
        env.storage().instance().set(&DataKey::Roles(account.clone(), role.clone()), &true);
        env.events().publish((Symbol::new(&env, "role_granted"),), (account, role));
    }
    
    /// Revoke a role from an address (admin only)
    pub fn revoke_role(env: Env, account: Address, role: Symbol) {
        let admin: Address = env.storage().instance().get(&DataKey::Admin).unwrap();
        admin.require_auth();
        
        env.storage().instance().set(&DataKey::Roles(account.clone(), role.clone()), &false);
        env.events().publish((Symbol::new(&env, "role_revoked"),), (account, role));
    }
    
    /// Mint NFT (requires minter role)
    pub fn mint(env: Env, to: Address, metadata: Bytes) -> u64 {
        let minter_role = Symbol::new(&env, "minter");
        
        // Check if caller has minter role or is admin
        let caller = env.current_contract_address(); // In real implementation, this would be msg.sender
        let admin: Address = env.storage().instance().get(&DataKey::Admin).unwrap();
        
        if !Self::has_role(env.clone(), caller.clone(), minter_role.clone()) && caller != admin {
            panic!("Caller does not have minter role");
        }
        
        let token_id: u64 = env.storage().instance().get(&DataKey::NextTokenId).unwrap_or(1);
        
        env.storage().instance().set(&DataKey::TokenOwner(token_id), &to);
        env.storage().instance().set(&DataKey::TokenMetadata(token_id), &metadata);
        env.storage().instance().set(&DataKey::NextTokenId, &(token_id + 1));
        
        env.events().publish((Symbol::new(&env, "nft_minted"),), (to, token_id));
        token_id
    }
    
    /// Burn NFT (requires burner role or owner)
    pub fn burn(env: Env, token_id: u64) {
        let burner_role = Symbol::new(&env, "burner");
        let caller = env.current_contract_address(); // In real implementation, this would be msg.sender
        let admin: Address = env.storage().instance().get(&DataKey::Admin).unwrap();
        
        let owner: Address = env.storage().instance().get(&DataKey::TokenOwner(token_id))
            .unwrap_or_else(|| panic!("Token does not exist"));
        
        // Check if caller has burner role, is admin, or is owner
        if !Self::has_role(env.clone(), caller.clone(), burner_role.clone()) 
            && caller != admin && caller != owner {
            panic!("Caller does not have permission to burn");
        }
        
        env.storage().instance().remove(&DataKey::TokenOwner(token_id));
        env.storage().instance().remove(&DataKey::TokenMetadata(token_id));
        
        env.events().publish((Symbol::new(&env, "nft_burned"),), token_id);
    }
    
    /// Get token owner
    pub fn owner_of(env: Env, token_id: u64) -> Address {
        env.storage().instance().get(&DataKey::TokenOwner(token_id))
            .unwrap_or_else(|| panic!("Token does not exist"))
    }
    
    /// Get token metadata
    pub fn token_metadata(env: Env, token_id: u64) -> Bytes {
        env.storage().instance().get(&DataKey::TokenMetadata(token_id))
            .unwrap_or_else(|| panic!("Token does not exist"))
    }
    
    /// Get contract name
    pub fn name(env: Env) -> Symbol {
        env.storage().instance().get(&DataKey::TokenName).unwrap()
    }
    
    /// Get contract symbol
    pub fn symbol(env: Env) -> Symbol {
        env.storage().instance().get(&DataKey::TokenSymbol).unwrap()
    }
    
    /// Get next token ID
    pub fn next_token_id(env: Env) -> u64 {
        env.storage().instance().get(&DataKey::NextTokenId).unwrap_or(1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::{testutils::Address as _, Address, Env, Bytes};

    #[test]
    fn test_access_control_nft() {
        let env = Env::default();
        let contract_id = env.register_contract(None, AccessControlNFT);
        let client = AccessControlNFTClient::new(&env, &contract_id);
        
        let admin = Address::generate(&env);
        let minter = Address::generate(&env);
        let user = Address::generate(&env);
        
        // Initialize contract
        client.initialize(&admin, &Symbol::new(&env, "AccessNFT"), &Symbol::new(&env, "ANFT"));
        
        // Test role management
        let minter_role = Symbol::new(&env, "minter");
        assert!(!client.has_role(&minter, &minter_role));
        
        client.grant_role(&minter, &minter_role);
        assert!(client.has_role(&minter, &minter_role));
        
        // Test NFT minting
        let metadata = Bytes::from_array(&env, &[1, 2, 3]);
        let token_id = client.mint(&user, &metadata);
        assert_eq!(token_id, 1);
        assert_eq!(client.owner_of(&token_id), user);
        
        // Test role revocation
        client.revoke_role(&minter, &minter_role);
        assert!(!client.has_role(&minter, &minter_role));
    }
}

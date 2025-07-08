// Simplified multi-extension token contract demonstrating multiple patterns
// Uses basic Soroban SDK features without OpenZeppelin dependencies
use soroban_sdk::{contract, contractimpl, contracttype, Env, Address, Symbol};

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Balance(Address),
    Allowance(Address, Address),
    Admin,
    Roles(Address, Symbol), // (address, role)
    Allowlist(Address),
    Blocklist(Address),
    Paused,
    TokenName,
    TokenSymbol,
    TokenDecimals,
    TotalSupply,
}

#[contracttype]
#[derive(Clone)]
pub enum MultiExtensionEvent {
    Transfer(Address, Address, i128),
    RoleGranted(Address, Symbol),
    RoleRevoked(Address, Symbol),
    UserAllowed(Address),
    UserBlocked(Address),
    Paused,
    Unpaused,
}

#[contract]
pub struct MultiExtensionToken;

#[contractimpl]
impl MultiExtensionToken {
    /// Initialize the token with multiple extension patterns
    pub fn initialize(
        env: Env,
        admin: Address,
        name: Symbol,
        symbol: Symbol,
        decimals: u32,
        total_supply: i128,
    ) {
        if env.storage().instance().has(&DataKey::Admin) {
            panic!("Already initialized");
        }
        
        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().instance().set(&DataKey::TokenName, &name);
        env.storage().instance().set(&DataKey::TokenSymbol, &symbol);
        env.storage().instance().set(&DataKey::TokenDecimals, &decimals);
        env.storage().instance().set(&DataKey::TotalSupply, &total_supply);
        env.storage().instance().set(&DataKey::Balance(admin.clone()), &total_supply);
        env.storage().instance().set(&DataKey::Paused, &false);
        
        // Admin gets all roles by default
        let admin_role = Symbol::new(&env, "admin");
        let minter_role = Symbol::new(&env, "minter");
        let pauser_role = Symbol::new(&env, "pauser");
        
        env.storage().instance().set(&DataKey::Roles(admin.clone(), admin_role), &true);
        env.storage().instance().set(&DataKey::Roles(admin.clone(), minter_role), &true);
        env.storage().instance().set(&DataKey::Roles(admin.clone(), pauser_role), &true);
        
        // Admin is automatically allowed
        env.storage().instance().set(&DataKey::Allowlist(admin.clone()), &true);
    }
    
    // === Access Control Functions ===
    
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
    
    // === Allowlist Functions ===
    
    /// Check if user is in allowlist
    pub fn allowed(env: Env, user: Address) -> bool {
        env.storage().instance().get(&DataKey::Allowlist(user)).unwrap_or(false)
    }
    
    /// Add user to allowlist (admin only)
    pub fn allow_user(env: Env, user: Address) {
        let admin: Address = env.storage().instance().get(&DataKey::Admin).unwrap();
        admin.require_auth();
        
        env.storage().instance().set(&DataKey::Allowlist(user.clone()), &true);
        env.events().publish((Symbol::new(&env, "user_allowed"),), user);
    }
    
    /// Remove user from allowlist (admin only)
    pub fn disallow_user(env: Env, user: Address) {
        let admin: Address = env.storage().instance().get(&DataKey::Admin).unwrap();
        admin.require_auth();
        
        env.storage().instance().set(&DataKey::Allowlist(user.clone()), &false);
        env.events().publish((Symbol::new(&env, "user_disallowed"),), user);
    }
    
    // === Blocklist Functions ===
    
    /// Check if user is blocked
    pub fn blocked(env: Env, user: Address) -> bool {
        env.storage().instance().get(&DataKey::Blocklist(user)).unwrap_or(false)
    }
    
    /// Add user to blocklist (admin only)
    pub fn block_user(env: Env, user: Address) {
        let admin: Address = env.storage().instance().get(&DataKey::Admin).unwrap();
        admin.require_auth();
        
        env.storage().instance().set(&DataKey::Blocklist(user.clone()), &true);
        env.events().publish((Symbol::new(&env, "user_blocked"),), user);
    }
    
    /// Remove user from blocklist (admin only)
    pub fn unblock_user(env: Env, user: Address) {
        let admin: Address = env.storage().instance().get(&DataKey::Admin).unwrap();
        admin.require_auth();
        
        env.storage().instance().set(&DataKey::Blocklist(user.clone()), &false);
        env.events().publish((Symbol::new(&env, "user_unblocked"),), user);
    }
    
    // === Pausable Functions ===
    
    /// Check if contract is paused
    pub fn paused(env: Env) -> bool {
        env.storage().instance().get(&DataKey::Paused).unwrap_or(false)
    }
    
    /// Pause the contract (pauser role required)
    pub fn pause(env: Env) {
        let pauser_role = Symbol::new(&env, "pauser");
        let caller = env.current_contract_address(); // In real implementation, this would be msg.sender
        
        if !Self::has_role(env.clone(), caller, pauser_role) {
            panic!("Caller does not have pauser role");
        }
        
        if Self::paused(env.clone()) {
            panic!("Contract already paused");
        }
        
        env.storage().instance().set(&DataKey::Paused, &true);
        env.events().publish((Symbol::new(&env, "paused"),), ());
    }
    
    /// Unpause the contract (pauser role required)
    pub fn unpause(env: Env) {
        let pauser_role = Symbol::new(&env, "pauser");
        let caller = env.current_contract_address(); // In real implementation, this would be msg.sender
        
        if !Self::has_role(env.clone(), caller, pauser_role) {
            panic!("Caller does not have pauser role");
        }
        
        if !Self::paused(env.clone()) {
            panic!("Contract not paused");
        }
        
        env.storage().instance().set(&DataKey::Paused, &false);
        env.events().publish((Symbol::new(&env, "unpaused"),), ());
    }
    
    // === Token Functions ===
    
    /// Transfer tokens with all security checks
    pub fn transfer(env: Env, from: Address, to: Address, amount: i128) {
        from.require_auth();
        
        // Check if contract is paused
        if Self::paused(env.clone()) {
            panic!("Contract is paused");
        }
        
        // Check allowlist
        if !Self::allowed(env.clone(), from.clone()) {
            panic!("From address not allowed");
        }
        if !Self::allowed(env.clone(), to.clone()) {
            panic!("To address not allowed");
        }
        
        // Check blocklist
        if Self::blocked(env.clone(), from.clone()) {
            panic!("From address is blocked");
        }
        if Self::blocked(env.clone(), to.clone()) {
            panic!("To address is blocked");
        }
        
        let from_balance: i128 = env.storage().instance().get(&DataKey::Balance(from.clone())).unwrap_or(0);
        if from_balance < amount {
            panic!("Insufficient balance");
        }
        
        let to_balance: i128 = env.storage().instance().get(&DataKey::Balance(to.clone())).unwrap_or(0);
        
        env.storage().instance().set(&DataKey::Balance(from.clone()), &(from_balance - amount));
        env.storage().instance().set(&DataKey::Balance(to.clone()), &(to_balance + amount));
        
        env.events().publish((Symbol::new(&env, "transfer"),), (from, to, amount));
    }
    
    /// Mint tokens (minter role required)
    pub fn mint(env: Env, to: Address, amount: i128) {
        let minter_role = Symbol::new(&env, "minter");
        let caller = env.current_contract_address(); // In real implementation, this would be msg.sender
        
        if !Self::has_role(env.clone(), caller, minter_role) {
            panic!("Caller does not have minter role");
        }
        
        if Self::paused(env.clone()) {
            panic!("Contract is paused");
        }
        
        let to_balance: i128 = env.storage().instance().get(&DataKey::Balance(to.clone())).unwrap_or(0);
        let total_supply: i128 = env.storage().instance().get(&DataKey::TotalSupply).unwrap();
        
        env.storage().instance().set(&DataKey::Balance(to.clone()), &(to_balance + amount));
        env.storage().instance().set(&DataKey::TotalSupply, &(total_supply + amount));
        
        env.events().publish((Symbol::new(&env, "mint"),), (to, amount));
    }
    
    /// Burn tokens (from own balance or with allowance)
    pub fn burn(env: Env, from: Address, amount: i128) {
        from.require_auth();
        
        if Self::paused(env.clone()) {
            panic!("Contract is paused");
        }
        
        let from_balance: i128 = env.storage().instance().get(&DataKey::Balance(from.clone())).unwrap_or(0);
        if from_balance < amount {
            panic!("Insufficient balance to burn");
        }
        
        let total_supply: i128 = env.storage().instance().get(&DataKey::TotalSupply).unwrap();
        
        env.storage().instance().set(&DataKey::Balance(from.clone()), &(from_balance - amount));
        env.storage().instance().set(&DataKey::TotalSupply, &(total_supply - amount));
        
        env.events().publish((Symbol::new(&env, "burn"),), (from, amount));
    }
    
    // === View Functions ===
    
    pub fn balance(env: Env, account: Address) -> i128 {
        env.storage().instance().get(&DataKey::Balance(account)).unwrap_or(0)
    }
    
    pub fn name(env: Env) -> Symbol {
        env.storage().instance().get(&DataKey::TokenName).unwrap()
    }
    
    pub fn symbol(env: Env) -> Symbol {
        env.storage().instance().get(&DataKey::TokenSymbol).unwrap()
    }
    
    pub fn decimals(env: Env) -> u32 {
        env.storage().instance().get(&DataKey::TokenDecimals).unwrap()
    }
    
    pub fn total_supply(env: Env) -> i128 {
        env.storage().instance().get(&DataKey::TotalSupply).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::{testutils::Address as _, Address, Env};

    #[test]
    fn test_multi_extension_token() {
        let env = Env::default();
        let contract_id = env.register_contract(None, MultiExtensionToken);
        let client = MultiExtensionTokenClient::new(&env, &contract_id);
        
        let admin = Address::generate(&env);
        let user1 = Address::generate(&env);
        let user2 = Address::generate(&env);
        
        // Initialize token
        client.initialize(
            &admin,
            &Symbol::new(&env, "MultiToken"),
            &Symbol::new(&env, "MT"),
            &18,
            &1000000,
        );
        
        // Test role-based access
        let minter_role = Symbol::new(&env, "minter");
        assert!(client.has_role(&admin, &minter_role));
        
        // Test allowlist
        assert!(client.allowed(&admin));
        assert!(!client.allowed(&user1));
        
        client.allow_user(&user1);
        assert!(client.allowed(&user1));
        
        // Test pausable
        assert!(!client.paused());
        client.pause();
        assert!(client.paused());
        
        client.unpause();
        assert!(!client.paused());
        
        // Test multi-pattern transfer
        client.allow_user(&user2);
        client.transfer(&admin, &user1, &1000);
        assert_eq!(client.balance(&user1), 1000);
    }
}

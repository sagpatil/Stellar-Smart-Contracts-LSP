// Simplified pausable token contract demonstrating pausable pattern
// Uses basic Soroban SDK features without OpenZeppelin dependencies
use soroban_sdk::{contract, contractimpl, contracttype, Env, Address, Symbol};

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Balance(Address),
    Allowance(Address, Address),
    Admin,
    Paused,
    TokenName,
    TokenSymbol,
    TokenDecimals,
    TotalSupply,
}

#[contracttype]
#[derive(Clone)]
pub enum PausableEvent {
    Paused,
    Unpaused,
    PausableTransfer(Address, Address, i128),
}

#[contract]
pub struct PausableToken;

#[contractimpl]
impl PausableToken {
    /// Initialize the token with pausable functionality
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
    }
    
    /// Check if contract is paused
    pub fn paused(env: Env) -> bool {
        env.storage().instance().get(&DataKey::Paused).unwrap_or(false)
    }
    
    /// Pause the contract (admin only)
    pub fn pause(env: Env) {
        let admin: Address = env.storage().instance().get(&DataKey::Admin).unwrap();
        admin.require_auth();
        
        if Self::paused(env.clone()) {
            panic!("Contract already paused");
        }
        
        env.storage().instance().set(&DataKey::Paused, &true);
        env.events().publish((Symbol::new(&env, "paused"),), ());
    }
    
    /// Unpause the contract (admin only)
    pub fn unpause(env: Env) {
        let admin: Address = env.storage().instance().get(&DataKey::Admin).unwrap();
        admin.require_auth();
        
        if !Self::paused(env.clone()) {
            panic!("Contract not paused");
        }
        
        env.storage().instance().set(&DataKey::Paused, &false);
        env.events().publish((Symbol::new(&env, "unpaused"),), ());
    }
    
    /// Transfer tokens (only when not paused)
    pub fn transfer(env: Env, from: Address, to: Address, amount: i128) {
        from.require_auth();
        
        // Check if contract is paused
        if Self::paused(env.clone()) {
            panic!("Contract is paused");
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
    
    /// Burn tokens (only when not paused, admin only)
    pub fn burn(env: Env, amount: i128) {
        let admin: Address = env.storage().instance().get(&DataKey::Admin).unwrap();
        admin.require_auth();
        
        if Self::paused(env.clone()) {
            panic!("Contract is paused");
        }
        
        let admin_balance: i128 = env.storage().instance().get(&DataKey::Balance(admin.clone())).unwrap_or(0);
        if admin_balance < amount {
            panic!("Insufficient balance to burn");
        }
        
        let total_supply: i128 = env.storage().instance().get(&DataKey::TotalSupply).unwrap();
        
        env.storage().instance().set(&DataKey::Balance(admin.clone()), &(admin_balance - amount));
        env.storage().instance().set(&DataKey::TotalSupply, &(total_supply - amount));
        
        env.events().publish((Symbol::new(&env, "burn"),), (admin, amount));
    }
    
    /// Get balance
    pub fn balance(env: Env, account: Address) -> i128 {
        env.storage().instance().get(&DataKey::Balance(account)).unwrap_or(0)
    }
    
    /// Get token name
    pub fn name(env: Env) -> Symbol {
        env.storage().instance().get(&DataKey::TokenName).unwrap()
    }
    
    /// Get token symbol
    pub fn symbol(env: Env) -> Symbol {
        env.storage().instance().get(&DataKey::TokenSymbol).unwrap()
    }
    
    /// Get decimals
    pub fn decimals(env: Env) -> u32 {
        env.storage().instance().get(&DataKey::TokenDecimals).unwrap()
    }
    
    /// Get total supply
    pub fn total_supply(env: Env) -> i128 {
        env.storage().instance().get(&DataKey::TotalSupply).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::{testutils::Address as _, Address, Env};

    #[test]
    fn test_pausable_functionality() {
        let env = Env::default();
        let contract_id = env.register_contract(None, PausableToken);
        let client = PausableTokenClient::new(&env, &contract_id);
        
        let admin = Address::generate(&env);
        let user1 = Address::generate(&env);
        
        // Initialize token
        client.initialize(
            &admin,
            &Symbol::new(&env, "PauseToken"),
            &Symbol::new(&env, "PST"),
            &18,
            &1000000,
        );
        
        // Test initial state
        assert!(!client.paused());
        
        // Test transfer when not paused
        client.transfer(&admin, &user1, &1000);
        assert_eq!(client.balance(&user1), 1000);
        
        // Pause contract
        client.pause();
        assert!(client.paused());
        
        // Unpause contract
        client.unpause();
        assert!(!client.paused());
    }
}

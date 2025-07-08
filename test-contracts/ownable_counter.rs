// Simplified ownable counter contract demonstrating ownable pattern
// Uses basic Soroban SDK features without OpenZeppelin dependencies
use soroban_sdk::{contract, contractimpl, contracttype, Env, Address, Symbol};

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Owner,
    Counter,
}

#[contracttype]
#[derive(Clone)]
pub enum OwnableEvent {
    OwnershipTransferred(Address, Address),
    CounterIncremented(i128),
    CounterDecremented(i128),
}

#[contract]
pub struct OwnableCounter;

#[contractimpl]
impl OwnableCounter {
    /// Initialize the counter with an owner
    pub fn initialize(env: Env, owner: Address) {
        if env.storage().instance().has(&DataKey::Owner) {
            panic!("Already initialized");
        }
        
        env.storage().instance().set(&DataKey::Owner, &owner);
        env.storage().instance().set(&DataKey::Counter, &0i128);
        
        env.events().publish((Symbol::new(&env, "initialized"),), owner);
    }
    
    /// Get the current owner
    pub fn owner(env: Env) -> Address {
        env.storage().instance().get(&DataKey::Owner).unwrap()
    }
    
    /// Transfer ownership (only current owner)
    pub fn transfer_ownership(env: Env, new_owner: Address) {
        let current_owner: Address = env.storage().instance().get(&DataKey::Owner).unwrap();
        current_owner.require_auth();
        
        env.storage().instance().set(&DataKey::Owner, &new_owner);
        env.events().publish((Symbol::new(&env, "ownership_transferred"),), (current_owner, new_owner));
    }
    
    /// Increment counter (only owner)
    pub fn increment(env: Env) {
        let owner: Address = env.storage().instance().get(&DataKey::Owner).unwrap();
        owner.require_auth();
        
        let current_value: i128 = env.storage().instance().get(&DataKey::Counter).unwrap_or(0);
        let new_value = current_value + 1;
        
        env.storage().instance().set(&DataKey::Counter, &new_value);
        env.events().publish((Symbol::new(&env, "incremented"),), new_value);
    }
    
    /// Decrement counter (only owner)
    pub fn decrement(env: Env) {
        let owner: Address = env.storage().instance().get(&DataKey::Owner).unwrap();
        owner.require_auth();
        
        let current_value: i128 = env.storage().instance().get(&DataKey::Counter).unwrap_or(0);
        let new_value = current_value - 1;
        
        env.storage().instance().set(&DataKey::Counter, &new_value);
        env.events().publish((Symbol::new(&env, "decremented"),), new_value);
    }
    
    /// Get current counter value (public)
    pub fn get_counter(env: Env) -> i128 {
        env.storage().instance().get(&DataKey::Counter).unwrap_or(0)
    }
    
    /// Reset counter to zero (only owner)
    pub fn reset(env: Env) {
        let owner: Address = env.storage().instance().get(&DataKey::Owner).unwrap();
        owner.require_auth();
        
        env.storage().instance().set(&DataKey::Counter, &0i128);
        env.events().publish((Symbol::new(&env, "reset"),), ());
    }
    
    /// Set counter to specific value (only owner)
    pub fn set_counter(env: Env, value: i128) {
        let owner: Address = env.storage().instance().get(&DataKey::Owner).unwrap();
        owner.require_auth();
        
        env.storage().instance().set(&DataKey::Counter, &value);
        env.events().publish((Symbol::new(&env, "set_counter"),), value);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::{testutils::Address as _, Address, Env};

    #[test]
    fn test_ownable_functionality() {
        let env = Env::default();
        let contract_id = env.register_contract(None, OwnableCounter);
        let client = OwnableCounterClient::new(&env, &contract_id);
        
        let owner = Address::generate(&env);
        let new_owner = Address::generate(&env);
        
        // Initialize contract
        client.initialize(&owner);
        assert_eq!(client.owner(), owner);
        assert_eq!(client.get_counter(), 0);
        
        // Test owner operations
        client.increment();
        assert_eq!(client.get_counter(), 1);
        
        client.decrement();
        assert_eq!(client.get_counter(), 0);
        
        client.set_counter(&42);
        assert_eq!(client.get_counter(), 42);
        
        client.reset();
        assert_eq!(client.get_counter(), 0);
        
        // Test ownership transfer
        client.transfer_ownership(&new_owner);
        assert_eq!(client.owner(), new_owner);
    }
}

// LSP Testing Contract
// This contract is designed to test various LSP features including
// completions, diagnostics, hover information, and go-to-definition

use soroban_sdk::{contract, contractimpl, contracttype, Env, Address, Symbol, Vec, Map, Bytes};

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Owner,
    Balances(Address),
    Paused,
    Roles(Address, Symbol),
    Config(Symbol),
    Counter,
    TestData(u64),
}

#[contracttype]
#[derive(Clone)]
pub enum TestEvent {
    Transfer(Address, Address, i128),
    RoleChanged(Address, Symbol, bool),
    StateChanged(Symbol, bool),
    CounterUpdated(i128),
}

#[contract]
pub struct LspTest;

#[contractimpl]
impl LspTest {
    /// Initialize the contract for LSP testing
    pub fn initialize(env: Env, owner: Address) {
        if env.storage().instance().has(&DataKey::Owner) {
            panic!("Already initialized");
        }
        
        env.storage().instance().set(&DataKey::Owner, &owner);
        env.storage().instance().set(&DataKey::Paused, &false);
        env.storage().instance().set(&DataKey::Counter, &0i128);
        
        // Set up initial roles
        let admin_role = Symbol::new(&env, "admin");
        env.storage().instance().set(&DataKey::Roles(owner.clone(), admin_role.clone()), &true);
        
        env.events().publish((Symbol::new(&env, "initialized"),), owner);
    }
    
    /// Test function for role-based access control
    pub fn grant_role(env: Env, account: Address, role: Symbol) {
        let owner: Address = env.storage().instance().get(&DataKey::Owner).unwrap();
        owner.require_auth();
        
        env.storage().instance().set(&DataKey::Roles(account.clone(), role.clone()), &true);
        env.events().publish((Symbol::new(&env, "role_granted"),), (account, role));
    }
    
    /// Test function for pausable functionality
    pub fn pause(env: Env) {
        let owner: Address = env.storage().instance().get(&DataKey::Owner).unwrap();
        owner.require_auth();
        
        if Self::paused(env.clone()) {
            panic!("Contract already paused");
        }
        
        env.storage().instance().set(&DataKey::Paused, &true);
        env.events().publish((Symbol::new(&env, "paused"),), ());
    }
    
    /// Test function for pausable functionality
    pub fn unpause(env: Env) {
        let owner: Address = env.storage().instance().get(&DataKey::Owner).unwrap();
        owner.require_auth();
        
        if !Self::paused(env.clone()) {
            panic!("Contract not paused");
        }
        
        env.storage().instance().set(&DataKey::Paused, &false);
        env.events().publish((Symbol::new(&env, "unpaused"),), ());
    }
    
    /// Test function demonstrating when_not_paused pattern
    pub fn transfer_when_not_paused(env: Env, from: Address, to: Address, amount: i128) {
        from.require_auth();
        
        // Check if contract is paused
        if Self::paused(env.clone()) {
            panic!("Contract is paused");
        }
        
        let from_balance: i128 = env.storage().instance().get(&DataKey::Balances(from.clone())).unwrap_or(0);
        if from_balance < amount {
            panic!("Insufficient balance");
        }
        
        let to_balance: i128 = env.storage().instance().get(&DataKey::Balances(to.clone())).unwrap_or(0);
        
        env.storage().instance().set(&DataKey::Balances(from.clone()), &(from_balance - amount));
        env.storage().instance().set(&DataKey::Balances(to.clone()), &(to_balance + amount));
        
        env.events().publish((Symbol::new(&env, "transfer"),), (from, to, amount));
    }
    
    /// Test function demonstrating when_paused pattern
    pub fn emergency_withdraw_when_paused(env: Env, account: Address) -> i128 {
        let owner: Address = env.storage().instance().get(&DataKey::Owner).unwrap();
        owner.require_auth();
        
        // Check if contract is paused
        if !Self::paused(env.clone()) {
            panic!("Contract must be paused for emergency withdrawal");
        }
        
        let balance: i128 = env.storage().instance().get(&DataKey::Balances(account.clone())).unwrap_or(0);
        env.storage().instance().set(&DataKey::Balances(account.clone()), &0);
        
        env.events().publish((Symbol::new(&env, "emergency_withdrawal"),), (account, balance));
        balance
    }
    
    /// Test function for counter operations
    pub fn increment_counter(env: Env) {
        let owner: Address = env.storage().instance().get(&DataKey::Owner).unwrap();
        owner.require_auth();
        
        let current: i128 = env.storage().instance().get(&DataKey::Counter).unwrap_or(0);
        let new_value = current + 1;
        
        env.storage().instance().set(&DataKey::Counter, &new_value);
        env.events().publish((Symbol::new(&env, "counter_incremented"),), new_value);
    }
    
    /// Test function for complex data operations
    pub fn store_test_data(env: Env, id: u64, data: Bytes) {
        let owner: Address = env.storage().instance().get(&DataKey::Owner).unwrap();
        owner.require_auth();
        
        env.storage().instance().set(&DataKey::TestData(id), &data);
        env.events().publish((Symbol::new(&env, "data_stored"),), (id, data));
    }
    
    /// Test function with complex return types
    pub fn get_user_info(env: Env, user: Address) -> Map<Symbol, i128> {
        let mut result = Map::new(&env);
        
        let balance: i128 = env.storage().instance().get(&DataKey::Balances(user.clone())).unwrap_or(0);
        result.set(Symbol::new(&env, "balance"), balance);
        
        let admin_role = Symbol::new(&env, "admin");
        let is_admin = env.storage().instance().get(&DataKey::Roles(user, admin_role)).unwrap_or(false);
        result.set(Symbol::new(&env, "is_admin"), if is_admin { 1 } else { 0 });
        
        result
    }
    
    /// Test function with vector operations
    pub fn bulk_transfer(env: Env, from: Address, recipients: Vec<Address>, amounts: Vec<i128>) {
        from.require_auth();
        
        if recipients.len() != amounts.len() {
            panic!("Recipients and amounts length mismatch");
        }
        
        if Self::paused(env.clone()) {
            panic!("Contract is paused");
        }
        
        let mut total_amount = 0i128;
        for amount in amounts.iter() {
            total_amount += amount;
        }
        
        let from_balance: i128 = env.storage().instance().get(&DataKey::Balances(from.clone())).unwrap_or(0);
        if from_balance < total_amount {
            panic!("Insufficient balance");
        }
        
        // Update sender balance
        env.storage().instance().set(&DataKey::Balances(from.clone()), &(from_balance - total_amount));
        
        // Update recipient balances
        for i in 0..recipients.len() {
            let recipient = recipients.get(i).unwrap();
            let amount = amounts.get(i).unwrap();
            let recipient_balance: i128 = env.storage().instance().get(&DataKey::Balances(recipient.clone())).unwrap_or(0);
            env.storage().instance().set(&DataKey::Balances(recipient.clone()), &(recipient_balance + amount));
        }
        
        env.events().publish((Symbol::new(&env, "bulk_transfer"),), (from, recipients, amounts));
    }
    
    // === View Functions ===
    
    pub fn paused(env: Env) -> bool {
        env.storage().instance().get(&DataKey::Paused).unwrap_or(false)
    }
    
    pub fn owner(env: Env) -> Address {
        env.storage().instance().get(&DataKey::Owner).unwrap()
    }
    
    pub fn has_role(env: Env, account: Address, role: Symbol) -> bool {
        env.storage().instance().get(&DataKey::Roles(account, role)).unwrap_or(false)
    }
    
    pub fn balance(env: Env, account: Address) -> i128 {
        env.storage().instance().get(&DataKey::Balances(account)).unwrap_or(0)
    }
    
    pub fn counter(env: Env) -> i128 {
        env.storage().instance().get(&DataKey::Counter).unwrap_or(0)
    }
    
    pub fn get_test_data(env: Env, id: u64) -> Option<Bytes> {
        env.storage().instance().get(&DataKey::TestData(id))
    }
    
    pub fn get_config(env: Env, key: Symbol) -> Option<i128> {
        env.storage().instance().get(&DataKey::Config(key))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::{testutils::Address as _, Address, Env, Vec, Bytes};

    #[test]
    fn test_lsp_contract_functionality() {
        let env = Env::default();
        let contract_id = env.register_contract(None, LSPTestContract);
        let client = LSPTestContractClient::new(&env, &contract_id);
        
        let owner = Address::generate(&env);
        let user1 = Address::generate(&env);
        let user2 = Address::generate(&env);
        
        // Initialize contract
        client.initialize(&owner);
        assert_eq!(client.owner(), owner);
        assert!(!client.paused());
        
        // Test role management
        let admin_role = Symbol::new(&env, "admin");
        assert!(client.has_role(&owner, &admin_role));
        
        client.grant_role(&user1, &admin_role);
        assert!(client.has_role(&user1, &admin_role));
        
        // Test pausable functionality
        client.pause();
        assert!(client.paused());
        
        client.unpause();
        assert!(!client.paused());
        
        // Test counter
        client.increment_counter();
        assert_eq!(client.counter(), 1);
        
        // Test data storage
        let test_data = Bytes::from_array(&env, &[1, 2, 3, 4]);
        client.store_test_data(&123, &test_data);
        assert_eq!(client.get_test_data(&123), Some(test_data));
        
        // Test bulk transfer
        let recipients = Vec::from_array(&env, [user1.clone(), user2.clone()]);
        let amounts = Vec::from_array(&env, [100i128, 200i128]);
        
        // First set balance for owner
        // This would normally be done through mint or initial setup
        // For test purposes, we'll just verify the function structure
        
        // Test user info
        let user_info = client.get_user_info(&owner);
        assert_eq!(user_info.get(Symbol::new(&env, "balance")), Some(0));
        assert_eq!(user_info.get(Symbol::new(&env, "is_admin")), Some(1));
    }
}

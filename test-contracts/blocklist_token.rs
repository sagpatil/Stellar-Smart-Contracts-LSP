// Simplified blocklist token contract demonstrating blocklist pattern
// Uses basic Soroban SDK features without OpenZeppelin dependencies
use soroban_sdk::{contract, contractimpl, contracttype, Env, Address, Symbol};

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Balance(Address),
    Allowance(Address, Address),
    Admin,
    Blocklist(Address),
    TokenName,
    TokenSymbol,
    TokenDecimals,
    TotalSupply,
}

#[contracttype]
#[derive(Clone)]
pub enum BlocklistEvent {
    UserBlocked(Address),
    UserUnblocked(Address),
    BlocklistTransfer(Address, Address, i128),
}

#[contract]
pub struct BlocklistToken;

#[contractimpl]
impl BlocklistToken {
    /// Initialize the token with blocklist functionality
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
    }
    
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
    
    /// Transfer tokens (blocked users cannot participate)
    pub fn transfer(env: Env, from: Address, to: Address, amount: i128) {
        from.require_auth();
        
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
    fn test_blocklist_functionality() {
        let env = Env::default();
        let contract_id = env.register_contract(None, BlocklistToken);
        let client = BlocklistTokenClient::new(&env, &contract_id);
        
        let admin = Address::generate(&env);
        let user1 = Address::generate(&env);
        let user2 = Address::generate(&env);
        
        // Initialize token
        client.initialize(
            &admin,
            &Symbol::new(&env, "BlockToken"),
            &Symbol::new(&env, "BLT"),
            &18,
            &1000000,
        );
        
        // Test blocklist
        assert!(!client.blocked(&user1));
        
        // Block user1
        client.block_user(&user1);
        assert!(client.blocked(&user1));
        
        // Test successful transfer (admin to user2)
        client.transfer(&admin, &user2, &1000);
        assert_eq!(client.balance(&user2), 1000);
        
        // Unblock user1
        client.unblock_user(&user1);
        assert!(!client.blocked(&user1));
    }
}

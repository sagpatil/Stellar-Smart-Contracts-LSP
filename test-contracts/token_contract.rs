use soroban_sdk::{contract, contractimpl, contracttype, Env, Address, Symbol};

#[contract]
pub struct TokenContract;

#[contracttype]
pub enum DataKey {
    Balance(Address),
    Nonce(Address),
    State(Address),
    Admin,
}

#[contracttype]
pub struct TokenMetadata {
    pub decimal: u32,
    pub name: Symbol,
    pub symbol: Symbol,
}

#[contractimpl]
impl TokenContract {
    pub fn initialize(env: Env, admin: Address, decimal: u32, name: Symbol, symbol: Symbol) {
        let metadata = TokenMetadata {
            decimal,
            name,
            symbol,
        };
        env.storage().instance().set(&DataKey::Admin, &admin);
    }

    pub fn balance(env: Env, id: Address) -> i128 {
        env.storage().persistent().get(&DataKey::Balance(id)).unwrap_or(0)
    }

    pub fn transfer(env: Env, from: Address, to: Address, amount: i128) {
        from.require_auth();
        
        let from_balance = Self::balance(env.clone(), from.clone());
        let to_balance = Self::balance(env.clone(), to.clone());
        
        env.storage().persistent().set(&DataKey::Balance(from), &(from_balance - amount));
        env.storage().persistent().set(&DataKey::Balance(to), &(to_balance + amount));
    }

    pub fn mint(env: Env, to: Address, amount: i128) {
        let admin: Address = env.storage().instance().get(&DataKey::Admin).unwrap();
        admin.require_auth();
        
        let balance = Self::balance(env.clone(), to.clone());
        env.storage().persistent().set(&DataKey::Balance(to), &(balance + amount));
    }
}

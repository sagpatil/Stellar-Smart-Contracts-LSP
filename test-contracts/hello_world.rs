use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Env, Symbol};

#[contract]
pub struct HelloContract;

#[contractimpl]
impl HelloContract {
    pub fn hello(env: Env, to: Symbol) -> Symbol {
        symbol_short!("Hello")
    }
}

#[contracttype]
pub enum DataKey {
    Counter,
}

#[contracttype]
pub struct State {
    pub count: u32,
    pub message: Symbol,
}

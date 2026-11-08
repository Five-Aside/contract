#![no_std]
//! FiveAsideReceipt — tamper-evident matchday / standings anchors.
//!
//! When a league publishes official week results, the backend hashes the
//! canonical standings + scorers JSON and calls publish_receipt. After the
//! cycle closes, settle_receipt records the final rollup hash.

use soroban_sdk::{contract, contractimpl, symbol, Address, BytesN, Env, String};

#[contract]
pub struct FiveAsideReceipt;

#[contractimpl]
impl FiveAsideReceipt {
    pub fn initialize(env: Env, admin: Address, operator: Address) {
        admin.require_auth();
        env.storage().instance().set(&symbol_short!("admin"), &admin);
        env.storage().instance().set(&symbol_short!("operator"), &operator);
    }

    pub fn publish_receipt(
        env: Env,
        league_id: String,
        cycle_key: String,
        week: u32,
        batch_hash: BytesN<32>,
    ) {
        let operator: Address = env.storage().instance().get(&symbol_short!("operator")).unwrap();
        operator.require_auth();
        let key = (symbol_short!("pub"), league_id.clone(), cycle_key.clone(), week);
        env.storage().persistent().set(&key, &batch_hash);
        env.events().publish((symbol_short!("publish"), league_id, cycle_key, week), batch_hash);
    }

    pub fn settle_receipt(
        env: Env,
        league_id: String,
        cycle_key: String,
        outcome_hash: BytesN<32>,
    ) {
        let operator: Address = env.storage().instance().get(&symbol_short!("operator")).unwrap();
        operator.require_auth();
        let key = (symbol_short!("set"), league_id.clone(), cycle_key.clone());
        env.storage().persistent().set(&key, &outcome_hash);
        env.events().publish((symbol_short!("settle"), league_id, cycle_key), outcome_hash);
    }
}

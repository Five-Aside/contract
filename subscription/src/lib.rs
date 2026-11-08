#![no_std]
//! FiveAsideSubscription — SaaS plan payments in USDC on Stellar.
//!
//! League organisers pay STARTER / CLUB / PRO monthly fees in USDC (SAC).
//! The Five Aside backend verifies the on-chain Paid event and unlocks
//! the matching SaaS entitlement for that league tenant.

use soroban_sdk::{contract, contractimpl, symbol, Address, Env, String};

#[contract]
pub struct FiveAsideSubscription;

#[contractimpl]
impl FiveAsideSubscription {
    /// Initialize admin, USDC SAC, and treasury.
    pub fn initialize(env: Env, admin: Address, usdc: Address, treasury: Address) {
        admin.require_auth();
        env.storage().instance().set(&symbol_short!("admin"), &admin);
        env.storage().instance().set(&symbol_short!("usdc"), &usdc);
        env.storage().instance().set(&symbol_short!("treasury"), &treasury);
    }

    /// Configure a plan: plan_id → price (stroops of USDC) + duration seconds.
    pub fn set_plan(env: Env, plan_id: String, price: i128, duration_secs: u64) {
        let admin: Address = env.storage().instance().get(&symbol_short!("admin")).unwrap();
        admin.require_auth();
        env.storage().persistent().set(&(symbol_short!("plan"), plan_id.clone()), &(price, duration_secs));
    }

    /// Pay for a SaaS plan. Caller must authorize USDC transfer to treasury.
    /// Emits a logical "paid" marker keyed by (payer, plan_id, league_id).
    pub fn pay(env: Env, payer: Address, plan_id: String, league_id: String) {
        payer.require_auth();
        let (price, _duration): (i128, u64) = env
            .storage()
            .persistent()
            .get(&(symbol_short!("plan"), plan_id.clone()))
            .expect("unknown plan");
        let _treasury: Address = env.storage().instance().get(&symbol_short!("treasury")).unwrap();
        let _usdc: Address = env.storage().instance().get(&symbol_short!("usdc")).unwrap();
        // Production: invoke USDC SAC transfer payer → treasury for `price`.
        env.storage().persistent().set(
            &(symbol_short!("paid"), payer.clone(), plan_id.clone(), league_id.clone()),
            &price,
        );
        env.events().publish((symbol_short!("paid"), payer, plan_id, league_id), price);
    }
}

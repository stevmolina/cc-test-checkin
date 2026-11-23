#![no_std]
//! CheckIn: an event check-in list on Stellar Testnet.
//!
//! Each attendee signs in once with their wallet and their name. The list of
//! who came is public and verifiable on-chain.

use soroban_sdk::{
    contract, contracterror, contractevent, contractimpl, contracttype, panic_with_error, Address,
    Env, String,
};

const DAY_IN_LEDGERS: u32 = 17_280;
const BUMP_THRESHOLD: u32 = 7 * DAY_IN_LEDGERS;
const BUMP_TO: u32 = 30 * DAY_IN_LEDGERS;

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Attendee {
    pub who: Address,
    pub name: String,
    pub ledger: u32,
}

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Count,
    Attendee(u32),
    Seen(Address),
}

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum Error {
    AlreadyCheckedIn = 1,
    BadName = 2,
}

#[contractevent]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CheckedIn {
    #[topic]
    pub who: Address,
    pub id: u32,
}

#[contract]
pub struct CheckIn;

#[contractimpl]
impl CheckIn {
    /// Register `who` at the event with their `name`. Only once per wallet.
    /// Returns the attendee number (starts at 1).
    pub fn check_in(env: Env, who: Address, name: String) -> u32 {
        who.require_auth();
        if name.len() == 0 || name.len() > 64 {
            panic_with_error!(&env, Error::BadName);
        }
        let seen = DataKey::Seen(who.clone());
        if env.storage().persistent().has(&seen) {
            panic_with_error!(&env, Error::AlreadyCheckedIn);
        }

        let id: u32 = env.storage().instance().get(&DataKey::Count).unwrap_or(0) + 1;
        let entry = Attendee {
            who: who.clone(),
            name,
            ledger: env.ledger().sequence(),
        };
        let key = DataKey::Attendee(id);
        env.storage().persistent().set(&key, &entry);
        env.storage().persistent().extend_ttl(&key, BUMP_THRESHOLD, BUMP_TO);
        env.storage().persistent().set(&seen, &id);
        env.storage().persistent().extend_ttl(&seen, BUMP_THRESHOLD, BUMP_TO);
        env.storage().instance().set(&DataKey::Count, &id);
        env.storage().instance().extend_ttl(BUMP_THRESHOLD, BUMP_TO);

        CheckedIn { who, id }.publish(&env);
        id
    }

    /// How many people checked in.
    pub fn count(env: Env) -> u32 {
        env.storage().instance().get(&DataKey::Count).unwrap_or(0)
    }

    /// Read one attendee by number.
    pub fn get(env: Env, id: u32) -> Option<Attendee> {
        env.storage().persistent().get(&DataKey::Attendee(id))
    }

    /// Has this wallet already checked in?
    pub fn has_checked_in(env: Env, who: Address) -> bool {
        env.storage().persistent().has(&DataKey::Seen(who))
    }
}

mod test;

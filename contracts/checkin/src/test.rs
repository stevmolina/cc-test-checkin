#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::Address as _, Address, Env, String};

fn setup() -> (Env, CheckInClient<'static>) {
    let env = Env::default();
    env.mock_all_auths();
    let id = env.register(CheckIn, ());
    let client = CheckInClient::new(&env, &id);
    (env, client)
}

#[test]
fn check_in_stores_attendee_and_counts() {
    let (env, client) = setup();
    let ana = Address::generate(&env);

    assert_eq!(client.count(), 0);
    assert!(!client.has_checked_in(&ana));
    let id = client.check_in(&ana, &String::from_str(&env, "Ana"));
    assert_eq!(id, 1);
    assert_eq!(client.count(), 1);
    assert!(client.has_checked_in(&ana));

    let a = client.get(&1).unwrap();
    assert_eq!(a.who, ana);
    assert_eq!(a.name, String::from_str(&env, "Ana"));
    assert!(client.get(&2).is_none());
}

#[test]
fn check_in_requires_auth() {
    let (env, client) = setup();
    let ana = Address::generate(&env);
    client.check_in(&ana, &String::from_str(&env, "Ana"));
    assert_eq!(env.auths()[0].0, ana);
}

#[test]
fn same_wallet_cannot_check_in_twice() {
    let (env, client) = setup();
    let ana = Address::generate(&env);
    client.check_in(&ana, &String::from_str(&env, "Ana"));
    let again = client.try_check_in(&ana, &String::from_str(&env, "Ana otra vez"));
    assert_eq!(again, Err(Ok(contract_err(Error::AlreadyCheckedIn))));
    assert_eq!(client.count(), 1);
}

#[test]
fn different_wallets_each_check_in() {
    let (env, client) = setup();
    client.check_in(&Address::generate(&env), &String::from_str(&env, "Ana"));
    let id = client.check_in(&Address::generate(&env), &String::from_str(&env, "Beto"));
    assert_eq!(id, 2);
    assert_eq!(client.get(&2).unwrap().name, String::from_str(&env, "Beto"));
}

#[test]
fn empty_name_is_rejected() {
    let (env, client) = setup();
    let ana = Address::generate(&env);
    let r = client.try_check_in(&ana, &String::from_str(&env, ""));
    assert_eq!(r, Err(Ok(contract_err(Error::BadName))));
}

fn contract_err(e: Error) -> soroban_sdk::Error {
    soroban_sdk::Error::from_contract_error(e as u32)
}

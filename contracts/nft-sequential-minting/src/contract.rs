//! Non-Fungible Vanilla Example Contract.
//!
//! Demonstrates an example usage of the NFT default base implementation.

use soroban_sdk::{Address, Env, String, contract, contractimpl, contracttype};
use stellar_tokens::non_fungible::{Base, NonFungibleToken, burnable::NonFungibleBurnable};

#[contracttype]
pub enum DataKey {
    Owner,
}

#[contract]
pub struct ExampleContract;

#[contractimpl]
impl ExampleContract {
    pub fn __constructor(e: &Env, uri: String, name: String, symbol: String, owner: Address) {
        e.storage().instance().set(&DataKey::Owner, &owner);
        Base::set_metadata(e, uri, name, symbol);
    }

    pub fn mint(e: &Env, to: Address) -> u32 {
        let owner: Address = e
            .storage()
            .instance()
            .get(&DataKey::Owner)
            .expect("owner should be set");
        owner.require_auth();
        Base::sequential_mint(e, &to)
    }
}

#[contractimpl(contracttrait)]
impl NonFungibleToken for ExampleContract {
    type ContractType = Base;
}

#[contractimpl(contracttrait)]
impl NonFungibleBurnable for ExampleContract {}

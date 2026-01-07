#![allow(dead_code)]

use soroban_sdk::{Address, Env, contracttrait};

// This file contains an interface for interacting with the NFT contract deployed by the gallery

#[contracttrait(client_name = "NftClient")]
pub trait NftInterface {
    fn mint(env: Env, to: Address);
    fn owner_of(e: &Env, token_id: u32) -> Address;
}

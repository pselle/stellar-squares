#![cfg(test)]

use crate::contract::{Contract, ContractClient};
use crate::nft::NftClient;
use soroban_sdk::testutils::Address as _;
use soroban_sdk::{Address, BytesN, Env, String};

const WASM: &[u8] = include_bytes!("../fixtures/nft_sequential_minting_example.wasm");

#[test]
fn test_deploy_collection() {
    let env = Env::default();
    env.mock_all_auths();

    let owner = Address::generate(&env);
    let wasm_hash: BytesN<32> = env.deployer().upload_contract_wasm(WASM);

    let contract_id = env.register(Contract, (owner, wasm_hash));
    let client = ContractClient::new(&env, &contract_id);

    let nft_address = client.deploy_collection(&String::from_str(&env, "https://example.com/"), &String::from_str(&env, "Squares Gallery"), &String::from_str(&env, "SQG"), &20u32);
    assert!(nft_address.exists());
    // Test collection_address getter
    assert!(client.collection_address(&String::from_str(&env, "SQG")) == nft_address);

    let nft_client = NftClient::new(&env, &nft_address);

    let first_nft_owner = nft_client.owner_of(&0u32);
    assert_eq!(&contract_id, &first_nft_owner);

    let last_nft_owner = nft_client.owner_of(&19u32);
    assert_eq!(&contract_id, &last_nft_owner);
}

#[test]
fn test_gallery_address() {
    let env = Env::default();
    env.mock_all_auths();

    let owner = Address::generate(&env);
    let wasm_hash: BytesN<32> = env.deployer().upload_contract_wasm(WASM);

    let contract_id = env.register(Contract, (owner, wasm_hash));
    let client = ContractClient::new(&env, &contract_id);

    let gallery_address = client.gallery_address();
    assert_eq!(gallery_address, contract_id);
}

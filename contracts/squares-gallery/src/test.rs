#![cfg(test)]

use crate::contract::{Contract, ContractClient};
use crate::nft::NftClient;
use soroban_sdk::testutils::Address as _;
use soroban_sdk::token::StellarAssetClient;
use soroban_sdk::{Address, BytesN, Env, String};

const WASM: &[u8] = include_bytes!("../fixtures/nft_sequential_minting_example.wasm");

#[test]
fn test_deploy_collection() {
    let env = Env::default();
    env.mock_all_auths();

    let owner = Address::generate(&env);
    let wasm_hash: BytesN<32> = env.deployer().upload_contract_wasm(WASM);
    let xlm_admin_address = Address::generate(&env);
    let xlm_sac = env.register_stellar_asset_contract_v2(xlm_admin_address);
    let contract_id = env.register(Contract, (owner, wasm_hash, xlm_sac.address()));
    let client = ContractClient::new(&env, &contract_id);

    let nft_address = client.deploy_collection(
        &String::from_str(&env, "https://example.com/"),
        &String::from_str(&env, "Squares Gallery"),
        &String::from_str(&env, "SQG"),
        &20u32,
    );
    assert!(nft_address.exists());
    // Test collection_address getter
    assert!(client.collection_address(&String::from_str(&env, "SQG")) == nft_address);

    let nft_client = NftClient::new(&env, &nft_address);

    let first_nft_owner = nft_client.owner_of(&0u32);
    assert_eq!(&contract_id, &first_nft_owner);

    let last_nft_owner = nft_client.owner_of(&19u32);
    assert_eq!(&contract_id, &last_nft_owner);

    // Deploy a second collection
    let nft_address_2 = client.deploy_collection(
        &String::from_str(&env, "https://example.com/"),
        &String::from_str(&env, "Squares Gallery 2"),
        &String::from_str(&env, "SQG2"),
        &10u32,
    );
    assert!(nft_address_2.exists());
    assert!(nft_address != nft_address_2);
}

#[test]
fn test_gallery_address() {
    let env = Env::default();
    env.mock_all_auths();

    let owner = Address::generate(&env);
    let wasm_hash: BytesN<32> = env.deployer().upload_contract_wasm(WASM);
    let xlm_admin_address = Address::generate(&env);
    let xlm_sac = env.register_stellar_asset_contract_v2(xlm_admin_address);
    let contract_id = env.register(Contract, (owner, wasm_hash, xlm_sac.address()));
    let client = ContractClient::new(&env, &contract_id);

    let gallery_address = client.gallery_address();
    assert_eq!(gallery_address, contract_id);
}

#[test]
fn test_purchase_nft() {
    let env = Env::default();
    env.mock_all_auths();
    let owner = Address::generate(&env);
    let buyer = Address::generate(&env);
    let wasm_hash: BytesN<32> = env.deployer().upload_contract_wasm(WASM);
    let xlm_admin_address = Address::generate(&env);
    let xlm_sac = env.register_stellar_asset_contract_v2(xlm_admin_address);

    // Mint XLM to our buyer
    let xlm_client = StellarAssetClient::new(&env, &xlm_sac.address());
    xlm_client.mint(&buyer, &100);

    let contract_id = env.register(Contract, (owner, wasm_hash, xlm_sac.address()));
    let client = ContractClient::new(&env, &contract_id);
    let nft_address = client.deploy_collection(
        &String::from_str(&env, "https://example.com/"),
        &String::from_str(&env, "Squares Gallery"),
        &String::from_str(&env, "SQG"),
        &5u32,
    );
    let nft_client = NftClient::new(&env, &nft_address);
    // The gallery owns token_id 2 initially
    let initial_owner = nft_client.owner_of(&2u32);
    assert_eq!(&contract_id, &initial_owner);
    // XLM balance of gallery is 0
    let gallery_xlm_balance = xlm_client.balance(&contract_id);
    assert_eq!(gallery_xlm_balance, 0);

    // Purchase token_id 2
    client.purchase_nft(&buyer.clone(), &String::from_str(&env, "SQG"), &2u32);
    let new_owner = nft_client.owner_of(&2u32);
    assert_eq!(&buyer, &new_owner);

    // Gallery now has 100 XLM
    let gallery_xlm_balance_after = xlm_client.balance(&contract_id);
    assert_eq!(gallery_xlm_balance_after, 100);
}

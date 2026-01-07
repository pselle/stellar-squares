#![cfg(test)]

use crate::contract::{Contract, ContractClient};
use crate::nft::NftClient;
use soroban_sdk::testutils::Address as _;
use soroban_sdk::{Address, BytesN, Env};

const WASM: &[u8] = include_bytes!("../fixtures/nft_sequential_minting_example.wasm");

#[test]
fn test() {
    let env = Env::default();
    env.mock_all_auths();

    let owner = Address::generate(&env);
    let wasm_hash: BytesN<32> = env.deployer().upload_contract_wasm(WASM);

    let contract_id = env.register(Contract, (owner, wasm_hash));
    let client = ContractClient::new(&env, &contract_id);

    let nft_address = client.initialize_collection();
    assert!(nft_address.exists());
    assert!(client.collection_address() == nft_address);

    let nft_client = NftClient::new(&env, &nft_address);

    let first_nft_owner = nft_client.owner_of(&0u32);
    assert_eq!(&contract_id, &first_nft_owner);

    let last_nft_owner = nft_client.owner_of(&19u32);
    assert_eq!(&contract_id, &last_nft_owner);
}

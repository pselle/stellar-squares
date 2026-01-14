use soroban_sdk::{
    Address, Bytes, BytesN, Env, String, contract, contracterror, contractimpl, contracttype,
    panic_with_error,
};

use crate::nft::NftClient;

const BASE_URI: &str =
    "https://bafybeid5jq6nn6m5tflzz5van67m74rt26c2z3tvowusxgio2qzmtkx6ki.ipfs.w3s.link/";
const NAME: &str = "Squares Gallery";
const SYMBOL: &str = "SGAL";

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    Unauthorized = 1,
    MintingFailed = 2,
}

#[contracttype]
pub enum DataKey {
    Owner,
    NftWasmHash,
    CollectionAddress,
}

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn __constructor(e: &Env, owner: Address, nft_wasm_hash: BytesN<32>) {
        e.storage().instance().set(&DataKey::Owner, &owner);
        e.storage()
            .instance()
            .set(&DataKey::NftWasmHash, &nft_wasm_hash);

        let nft_wasm_hash: BytesN<32> = e
            .storage()
            .instance()
            .get(&DataKey::NftWasmHash)
            .expect("nft_wasm_hash should be set");

        // Return the contract ID
        let contract_id = e
            .deployer()
            .with_current_contract(e.crypto().sha256(&Bytes::new(e)))
            .deploy_v2(
                nft_wasm_hash,
                (
                    String::from_str(e, BASE_URI), // base_uri
                    String::from_str(e, NAME),     // name
                    String::from_str(e, SYMBOL),   // symbol
                    e.current_contract_address(),  // owner
                ),
            );
        e.storage()
            .instance()
            .set(&DataKey::CollectionAddress, &contract_id);
    }

    pub fn collection_address(e: &Env) -> Address {
        e.storage()
            .instance()
            .get(&DataKey::CollectionAddress)
            .expect("nft should be set")
    }

    pub fn gallery_address(e: &Env) -> Address {
        e.current_contract_address()
    }

    pub fn initialize_collection(e: &Env) -> Address {
        let owner: Address = e
            .storage()
            .instance()
            .get(&DataKey::Owner)
            .expect("owner should be set");
        owner.require_auth();

        let contract_id: Address = Self::collection_address(e);

        let client = NftClient::new(e, &contract_id);
        // Mint the 20 NFTs of the collection to the gallery contract itself
        for _ in 0..20 {
            let result = client.try_mint(&e.current_contract_address());
            match result {
                Ok(_) => (),
                Err(_) => panic_with_error!(e, Error::MintingFailed),
            }
        }
        contract_id
    }
}

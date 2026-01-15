use soroban_sdk::{
    Address, BytesN, Env, String, contract, contracterror, contractimpl, contracttype, log,
    panic_with_error,
};

use crate::nft::NftClient;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    Unauthorized = 1,
    MintingFailed = 2,
    SymbolAlreadyDeployed = 3,
}

#[contracttype]
pub enum DataKey {
    Owner,
    NftWasmHash,
    CollectionAddress(String), // Keyed by collection symbol, which is stored as a String on the NFT contract standard
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
    }

    pub fn collection_address(e: &Env, symbol: String) -> Address {
        e.storage()
            .instance()
            .get(&DataKey::CollectionAddress(symbol))
            .expect("collection_address not present for symbol")
    }

    pub fn gallery_address(e: &Env) -> Address {
        e.current_contract_address()
    }

    /// Deploys a new NFT collection contract with the given parameters and mints the specified quantity
    /// of NFTs to the gallery contract itself.
    pub fn deploy_collection(
        e: &Env,
        base_uri: String,
        name: String,
        symbol: String,
        collection_size: u32,
    ) -> Address {
        let owner: Address = e
            .storage()
            .instance()
            .get(&DataKey::Owner)
            .expect("owner should be set");
        owner.require_auth();

        // Ensure symbol is not already used
        if e.storage()
            .instance()
            .has(&DataKey::CollectionAddress(symbol.clone()))
        {
            panic_with_error!(e, Error::SymbolAlreadyDeployed);
        }

        let nft_wasm_hash: BytesN<32> = e
            .storage()
            .instance()
            .get(&DataKey::NftWasmHash)
            .expect("nft_wasm_hash should be set");

        // Generate salt based on the symbol to ensure unique contract address per symbol
        let salt = e.crypto().sha256(&symbol.to_bytes());
        let collection_address = e.deployer().with_current_contract(salt).deploy_v2(
            nft_wasm_hash,
            (
                base_uri,
                name,
                symbol.clone(),
                e.current_contract_address(), // owner
            ),
        );

        e.storage().instance().set(
            &DataKey::CollectionAddress(symbol.clone()),
            &collection_address,
        );

        let client = NftClient::new(e, &collection_address);
        // Mint the N number of NFTs of the collection to the gallery contract itself
        for _ in 0..collection_size {
            let result = client.try_mint(&e.current_contract_address());
            match result {
                Ok(_) => (),
                Err(_) => panic_with_error!(e, Error::MintingFailed),
            }
        }
        collection_address
    }
}

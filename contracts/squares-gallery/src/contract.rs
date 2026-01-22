use soroban_sdk::{
    Address, BytesN, Env, String, contract, contracterror, contractimpl, contracttype,
    panic_with_error, token::TokenClient,
};

use crate::nft::NftClient;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    Unauthorized = 1,
    MintingFailed = 2,
    SymbolAlreadyDeployed = 3,
    XLMTransferFailed = 4,
    TokenNotOwnedByGallery = 5,
    InvalidWithdrawalAmount = 6,
}

type CollectionSymbol = String;

#[contracttype]
pub enum DataKey {
    Owner,
    NftWasmHash,
    XlmSac,
    CollectionAddress(CollectionSymbol), // Keyed by collection symbol, which is stored as a String on the NFT contract standard
    ItemPrice(CollectionSymbol),         // Initial price of items in the collection
}

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn __constructor(e: &Env, owner: Address, nft_wasm_hash: BytesN<32>, xlm_sac: Address) {
        e.storage().instance().set(&DataKey::Owner, &owner);
        e.storage()
            .instance()
            .set(&DataKey::NftWasmHash, &nft_wasm_hash);
        e.storage().instance().set(&DataKey::XlmSac, &xlm_sac);
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
        item_price: i128,
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
        // Store the item price for this collection
        e.storage().instance().set(&DataKey::ItemPrice(symbol.clone()), &item_price);
        collection_address
    }

    // Purchase NFT from collection by symbol and token_id
    pub fn purchase_nft(e: &Env, buyer: Address, symbol: String, token_id: u32) {
        // Buyer must authorize the purchase
        buyer.require_auth();
        let gallery_address = e.current_contract_address();
        let collection_address: Address = e
            .storage()
            .instance()
            .get(&DataKey::CollectionAddress(symbol.clone()))
            .expect("collection_address not present for symbol");
        let client = NftClient::new(e, &collection_address);

        // Ensure that token is owned by gallery
        let owner = client.owner_of(&token_id);
        if owner != gallery_address {
            panic_with_error!(e, Error::TokenNotOwnedByGallery);
        }

        // Purchaser transfers the item price in XLM to gallery for the purchase
        let item_price: i128 = e
            .storage()
            .instance()
            .get(&DataKey::ItemPrice(symbol.clone()))
            .expect("item_price not present for symbol");

        let _ = Self::xlm_client(e)
            .try_transfer(&buyer, &gallery_address, &item_price)
            .unwrap_or_else(|_| panic_with_error!(e, Error::XLMTransferFailed));

        // Transfer the NFT from gallery to buyer
        client.transfer(&gallery_address, &buyer, &token_id);
    }

    pub fn withdraw(e: &Env, amount: i128) {
        let owner: Address = e
            .storage()
            .instance()
            .get(&DataKey::Owner)
            .expect("owner should be set");
        // Only owner can request withdrawals
        owner.require_auth();

        // get the balance of the gallery contract
        let gallery_address = e.current_contract_address();
        let balance = Self::xlm_client(e).balance(&gallery_address);
        // Assert that the amount is less than or equal to the balance
        if amount > balance {
            panic_with_error!(e, Error::InvalidWithdrawalAmount);
        }

        let gallery_address = e.current_contract_address();
        let _ = Self::xlm_client(e)
            .try_transfer(&gallery_address, &owner, &amount)
            .unwrap_or_else(|_| panic_with_error!(e, Error::XLMTransferFailed));
    }

    fn xlm_client(env: &Env) -> TokenClient<'_> {
        let xlm_sac: Address = env
            .storage()
            .instance()
            .get(&DataKey::XlmSac)
            .expect("xlm_sac should be set");
        TokenClient::new(env, &xlm_sac)
    }
}

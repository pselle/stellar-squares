// Typed client for the NFT collections the gallery deploys, generated at build
// time from OpenZeppelin's `nft-standard` Wasm published in the Stellar Registry.

// The macro expands to code that reaches for `super::soroban_sdk`.
#[allow(clippy::single_component_path_imports)]
use soroban_sdk;

stellar_registry::import_contract_client!("oz/nft-standard@0.7.2");

pub use nft_standard::Client as NftClient;
#[cfg(test)]
pub use nft_standard::WASM;

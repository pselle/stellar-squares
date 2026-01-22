# Stellar Squares NFT Collection

A gallery of NFT art and their associated owners. If the gallery owns the NFT in the collection, you can buy it!

This is a [Scaffold Stellar](https://scaffoldstellar.org) project using audited an [OpenZeppelin contract](https://github.com/OpenZeppelin/stellar-contracts) for the NFT contract.

## Requirements

Before getting started, make sure you’ve met the requirements listed in the [Soroban documentation](https://developers.stellar.org/docs/build/smart-contracts/getting-started/setup) and that the following tools are installed :

- [Rust](https://www.rust-lang.org/tools/install)
- [Cargo](https://doc.rust-lang.org/cargo/) (comes with Rust)
- Rust target: install the compilation target listed in the [Soroban setup guide](https://developers.stellar.org/docs/build/smart-contracts/getting-started/setup)
- [Node.js](https://nodejs.org/en/download/package-manager) (v22, or higher)
- [npm](https://www.npmjs.com/): Comes with the node installer or can also be installed package managers such as Homebrew, Chocolatey, apt, etc.
- [Stellar CLI](https://github.com/stellar/stellar-core)
- [Scaffold Stellar CLI Plugin](https://github.com/AhaLabs/scaffold-stellar)

## Running the project locally

The local environment (and others) are defined in environments.toml.

Scaffold Stellar will set-up the keys and install WASM, however to initalize the gallery contract, you'll need to upload the NFT wasm fixture to your local environment:

```
stellar network use local
stellar contract upload --wasm contracts/squares-gallery/fixtures/nft_sequential_minting_example.wasm
# Copy the hash of the upload output
```

Copy the hash from this upload and update this section of environments.toml:

```
[development.contracts.squares_gallery]
...
constructor_args = "--owner me --nft_wasm_hash YOUR_WASM_HASH_HERE --xlm_sac $(stellar contract id asset --asset native)"
```

With an appropriate wasm hash present, you will be able to run `npm run start` and run the project locally like any other Scaffold Stellar project.

## Deploying the project

The project is deployed to Vercel as a static build. To deploy a staging (testnet) site from your local environment:

```
npm run build:staging
npx vercel build
npx vercel --prebuilt
```

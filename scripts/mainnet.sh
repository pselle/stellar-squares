export STELLAR_NETWORK_PASSPHRASE="Public Global Stellar Network ; September 2015"
export STELLAR_RPC_URL="https://rpc.lightsail.network/"
export STELLAR_ACCOUNT=gallery-mainnet
export STELLAR_NETWORK=mainnet

rm -rf target/stellar
stellar scaffold build production
stellar contract optimize --wasm target/stellar/mainnet/squares_gallery.wasm

# Upload OZ NFT contract. v2 will replace this with references to the Stellar Registry
WASM_HASH=$(stellar contract upload --wasm contracts/squares-gallery/fixtures/nft_sequential_minting_example.wasm --source-account gallery-mainnet)
# Deploy the gallery contract
contract_id=$(stellar contract deploy --wasm target/stellar/mainnet/squares_gallery.optimized.wasm --source gallery-mainnet -- --owner gallery-mainnet --nft_wasm_hash $WASM_HASH --xlm_sac $(stellar contract id asset --asset native))
echo "Deployed squares_gallery contract with ID: $contract_id"
# Update contract ID in environments.toml
awk -v id="$contract_id" '
  BEGIN { in_production=0 }
  /^\[production\.contracts\]/ { in_production=1 }
  /^\[/ && $0 !~ /^\[production\.contracts\]/ { in_production=0 }
  in_production && /^squares_gallery = { id = / {
    print "squares_gallery = { id = \"" id "\" }"
    next
  }
  { print }
' environments.toml > environments.toml.tmp && mv environments.toml.tmp environments.toml
echo "Updated environments.toml with new gallery contract ID."

# Deploy the Squares NFT collection
stellar contract invoke --id $contract_id  --source gallery-mainnet -- deploy_collection --base_uri "ipfs://bafybeicqgwje7trm27thcwngfhtz2ppadly2zcnxp3ch6plt5fe4ipoacu/" --name "Stellar Squares" --symbol "SSQ" --collection_size 20 --item_price 100

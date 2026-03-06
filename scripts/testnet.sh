STELLAR_NETWORK="TESTNET"
STELLAR_NETWORK_PASSPHRASE="Test SDF Network ; September 2015"
STELLAR_RPC_URL="https://soroban-testnet.stellar.org"
STELLAR_HORIZON_URL="https://horizon-testnet.stellar.org"

rm -rf target/stellar
stellar scaffold build staging
stellar contract optimize --wasm target/stellar/testnet/squares_gallery.wasm

# If we passed an "upgrade" argument, we'll upgrade the existing contract instead of deploying a new one
if [ "$1" = "upgrade" ]; then
  existing_contract_id=$(awk -F'"' '/squares_gallery = { id = "/ {print $2}' environments.toml | head -n 1)
  if [ -z "$existing_contract_id" ]; then
    echo "No existing contract ID found in environments.toml. Cannot perform upgrade."
    exit 1
  fi
  # echo "Upgrading existing contract with ID: $existing_contract_id"
  NEW_WASM_HASH=$(stellar contract upload --wasm target/stellar/testnet/squares_gallery.optimized.wasm --source-account testnet-user)
  # echo "New WASM uploaded with hash: $NEW_WASM_HASH"
  stellar contract invoke --id $existing_contract_id --source testnet-user -- upgrade --new-wasm-hash $NEW_WASM_HASH
  echo "Upgrade transaction submitted. Please check the contract's transaction history for confirmation."
  exit 0
fi
# Upload OZ NFT contract. v2 will replace this with references to the Stellar Registry
WASM_HASH=$(stellar contract upload --wasm contracts/squares-gallery/fixtures/nft_sequential_minting_example.wasm --source-account testnet-user)
# Deploy the gallery contract
contract_id=$(stellar contract deploy --wasm target/stellar/testnet/squares_gallery.optimized.wasm --source testnet-user -- --owner testnet-user --nft_wasm_hash $WASM_HASH --xlm_sac $(stellar contract id asset --asset native))
echo "Deployed squares_gallery contract with ID: $contract_id"
# Update contract ID in environments.toml
awk -v id="$contract_id" '
  BEGIN { in_staging=0 }
  /^\[staging\.contracts\]/ { in_staging=1 }
  /^\[/ && $0 !~ /^\[staging\.contracts\]/ { in_staging=0 }
  in_staging && /^squares_gallery = { id = / {
    print "squares_gallery = { id = \"" id "\" }"
    next
  }
  { print }
' environments.toml > environments.toml.tmp && mv environments.toml.tmp environments.toml
echo "Updated environments.toml with new gallery contract ID."

# Deploy the Squares NFT collection
item_price=1000000000 # 100 XLM in stroops
collection_id=$(stellar contract invoke --id $contract_id  --source testnet-user -- deploy_collection --base_uri "ipfs://bafybeicqgwje7trm27thcwngfhtz2ppadly2zcnxp3ch6plt5fe4ipoacu/" --name "Stellar Squares" --symbol "SSQ" --collection_size 20 --item_price $item_price)

# Update environments.toml to point to that collection for the nft example contract
awk -v nft_id="$collection_id" '
  BEGIN { in_staging=0 }
  /^\[staging\.contracts\]/ { in_staging=1 }
  /^\[/ && $0 !~ /^\[staging\.contracts\]/ { in_staging=0 }
  in_staging && /^nft_sequential_minting_example = { id = / {
    print "nft_sequential_minting_example = { id = " nft_id " }"
    next
  }
  { print }
' environments.toml > environments.toml.tmp && mv environments.toml.tmp environments.toml

echo "Updated environments.toml with new NFT collection contract ID."
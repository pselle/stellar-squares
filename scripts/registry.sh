#!/usr/bin/env bash
# Publish, deploy and upgrade the gallery through the Stellar Registry.
#
#   scripts/registry.sh <testnet|mainnet> publish      build the gallery and publish its Wasm
#   scripts/registry.sh <testnet|mainnet> deploy       deploy a named gallery from the published Wasm
#   scripts/registry.sh <testnet|mainnet> upgrade      upgrade the gallery to the latest published Wasm
#   scripts/registry.sh <testnet|mainnet> collection   have the gallery deploy the Stellar Squares collection
#   scripts/registry.sh <testnet|mainnet> id           print the gallery's contract ID
#
# Extra arguments are passed to the underlying command, e.g. `publish --dry-run`.
# The published version is the `version` in contracts/squares-gallery/Cargo.toml.
set -euo pipefail

# `unverified/` is the Registry's open channel; a verified name needs manager approval.
GALLERY_NAME="unverified/squares-gallery"
# The Wasm the gallery deploys collections from
NFT_WASM_NAME="oz/nft-standard"
NFT_WASM_VERSION="0.7.2"
# Price of each square. Set it in XLM; the contract takes stroops.
#   1 XLM = 10,000,000 stroops, so 25 XLM = 250,000,000 stroops
ITEM_PRICE_XLM=25
ITEM_PRICE=$((ITEM_PRICE_XLM * 10000000))

usage() {
  echo "Usage: $0 <testnet|mainnet> <publish|deploy|upgrade|collection|id>" >&2
  exit 1
}

[ $# -ge 2 ] || usage
network="$1"
command="$2"
shift 2

case "$network" in
testnet)
  export STELLAR_NETWORK=testnet
  export STELLAR_ACCOUNT="${STELLAR_ACCOUNT:-testnet-user}"
  export STELLAR_SCAFFOLD_ENV=staging
  ;;
mainnet)
  export STELLAR_NETWORK=mainnet
  export STELLAR_NETWORK_PASSPHRASE="Public Global Stellar Network ; September 2015"
  export STELLAR_RPC_URL="${STELLAR_RPC_URL:-https://rpc.lightsail.network/}"
  export STELLAR_ACCOUNT="${STELLAR_ACCOUNT:-gallery-mainnet}"
  export STELLAR_SCAFFOLD_ENV=production
  ;;
*)
  usage
  ;;
esac

gallery_id() {
  stellar registry fetch-contract-id "$GALLERY_NAME"
}

case "$command" in
publish)
  stellar scaffold build
  stellar registry publish \
    --wasm "target/stellar/$STELLAR_NETWORK/squares_gallery.wasm" \
    --wasm-name "$GALLERY_NAME" \
    "$@"
  ;;
deploy)
  stellar registry deploy \
    --contract-name "$GALLERY_NAME" \
    --wasm-name "$GALLERY_NAME" \
    "$@" \
    -- \
    --owner "$STELLAR_ACCOUNT" \
    --nft_wasm_hash "$(stellar registry fetch-hash "$NFT_WASM_NAME" --version "$NFT_WASM_VERSION")" \
    --xlm_sac "$(stellar contract id asset --asset native)"
  echo "Deployed $GALLERY_NAME with ID: $(gallery_id)"
  echo "Set this ID for squares_gallery in environments.toml so the app uses it."
  ;;
upgrade)
  stellar registry upgrade \
    --contract-name "$GALLERY_NAME" \
    --wasm-name "$GALLERY_NAME" \
    "$@"
  ;;
collection)
  stellar contract invoke --id "$(gallery_id)" "$@" -- deploy_collection \
    --base_uri "ipfs://bafybeicqgwje7trm27thcwngfhtz2ppadly2zcnxp3ch6plt5fe4ipoacu/" \
    --name "Stellar Squares" \
    --symbol "SSQ" \
    --collection_size 20 \
    --item_price "$ITEM_PRICE"
  ;;
id)
  gallery_id
  ;;
*)
  usage
  ;;
esac

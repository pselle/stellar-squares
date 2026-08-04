# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## What this is

Stellar Squares is a generative-art NFT gallery dApp built with Scaffold Stellar. A React/TypeScript
frontend talks to two Soroban smart contracts: a "gallery" contract (this repo's own code) that deploys
and sells NFT collections, and an OpenZeppelin-based NFT contract (a vendored example) that the gallery
deploys per-collection.

## Commands

Frontend (Node v22+):
- `npm start` — runs `stellar scaffold watch --build-clients` and `vite` concurrently. This is the main
  dev loop: it rebuilds contracts, regenerates TypeScript client bindings into `packages/` and
  `src/contracts/`, and reruns Vite on changes. App runs at `http://localhost:5173`.
- `npm run build` — `tsc -b && vite build` (production, uses `.env`/default env).
- `npm run build:staging` — builds contracts for the `staging` scaffold environment and Vite in
  `staging` mode (uses `.env.staging`).
- `npm run lint` — ESLint over the whole repo.
- `npm run format` — Prettier write.
- `npm run install:contracts` — installs and builds the `packages/*` workspace (generated contract
  client packages).
- `npx prettier . --check` — what CI runs (not `format`, which writes).

Contracts (Rust/Soroban):
- `cargo test` (from repo root, workspace) — runs all contract unit tests.
- `cargo test -p squares-gallery` / `cargo test -p nft-sequential-minting-example` — test a single
  contract crate.
- `cargo test <test_name>` — run a single test function by name.
- Rust toolchain is pinned via `rust-toolchain.toml` (1.89.0, `wasm32v1-none` target) — no manual
  toolchain selection needed if `rustup` respects it.

Deploying/redeploying contracts (not part of normal frontend dev — only needed when
`environments.toml` contract IDs or WASM need to change):
- `scripts/testnet.sh` — builds, uploads, deploys gallery + a fresh 20-item collection to testnet, and
  rewrites the relevant IDs in `environments.toml`. `scripts/testnet.sh upgrade` instead upgrades the
  existing testnet gallery contract in place (uses the `upgrade` entrypoint, no new collection).
- `scripts/mainnet.sh` — same shape, for mainnet, using the `gallery-mainnet` Stellar CLI identity.

There is no frontend test suite currently (`npm test --if-present` in CI is a no-op).

## Architecture

### Contract layer (`contracts/`)

Two-contract split, both Soroban (`no_std`) crates in a Cargo workspace (`contracts/*`):

- **`squares-gallery`** — the project's actual contract (`contract.rs`). Owner-controlled. Key behavior:
  - `deploy_collection`: deploys a new instance of the NFT wasm (identified by `nft_wasm_hash`, set at
    construction) via `deployer().with_current_contract(salt)`, where `salt` = sha256 of the collection
    `symbol`. This makes the collection's address deterministic per-symbol and prevents redeploying
    under the same symbol (`Error::SymbolAlreadyDeployed`). It then mints `collection_size` NFTs to
    itself (the gallery) and records `item_price` for that symbol.
  - `purchase_nft`: buyer `require_auth()`s, gallery verifies it still owns the token, moves the buyer's
    XLM (via `TokenClient` against the configured `xlm_sac`) to the gallery, then transfers the NFT.
  - `withdraw` / `upgrade`: owner-only; `upgrade` calls `deployer().update_current_contract_wasm` —
    the *new* wasm must accept the same constructor args since constructor state isn't rerun.
  - `nft.rs` defines a **local trait interface** (`NftInterface`/`NftClient`) for calling into the
    deployed NFT contract by address — the gallery doesn't depend on the NFT crate directly, it just
    knows the subset of its interface (`mint`, `owner_of`, `transfer`) it needs to call cross-contract.
  - Storage is all `instance()` storage, keyed by a `DataKey` enum; collection address and item price are
    both keyed by collection `symbol` (`CollectionAddress(String)`, `ItemPrice(String)`), so the gallery
    supports multiple collections even though only one (`SSQ`) is deployed today.

- **`nft-sequential-minting`** — a vendored/example OpenZeppelin NFT contract
  (`ExampleContract`/`nft_sequential_minting_example`), built on `stellar-tokens::non_fungible::Base`.
  Owner-gated `mint` using `Base::sequential_mint`. This is what the gallery deploys as each
  collection's contract; its compiled `.wasm` is checked into
  `contracts/squares-gallery/fixtures/nft_sequential_minting_example.wasm` and referenced by hash in
  `environments.toml` / `scripts/*.sh` rather than deployed straight from source. When this contract's
  source changes, the fixture wasm must be rebuilt and the hash updated everywhere it's referenced.

`environments.toml` defines per-environment (`development`/`staging`/`production`) network config,
Stellar CLI account identities, contract constructor args, and an `after_deploy` hook (dev only) that
calls `deploy_collection` automatically. Contract IDs here for staging/production are updated by
`scripts/testnet.sh` / `scripts/mainnet.sh`, not edited by hand in normal operation.

### Frontend (`src/`)

- **Generated contract clients are not checked in.** `packages/` (TS client packages, one per
  contract, workspace-installed) and everything under `src/contracts/` except `util.ts` are produced by
  `stellar scaffold watch --build-clients` / `stellar-scaffold build --build-clients` from the deployed
  contracts + `environments.toml`. Code imports them as ordinary packages, e.g.
  `import squaresGallery from "../contracts/squares_gallery"` and
  `import { Client } from "nft_sequential_minting_example"`. If these imports don't resolve, the fix is
  to run the build/watch command, not to hand-write the missing files.
- **`src/contracts/util.ts`** is the one hand-written exception in that directory — shared `rpcUrl` /
  `networkPassphrase` re-exports consumed by the generated clients and by hooks. Environment/network
  config is actually validated in `src/util/contract.ts` via a `zod` schema over `import.meta.env`
  (`PUBLIC_*` vars, see `envPrefix: "PUBLIC_"` in `vite.config.ts`), with a LOCAL-network fallback if
  parsing fails.
- **Data fetching**: `src/hooks/useNftCollection.ts` wraps generated-client calls in TanStack Query
  hooks (`useGetCollectionAddress`, `useGetGalleryAddress`, `useGetOwner`, `useGetTokenUri`). A single
  module-level `nftClient` is memoized per collection address rather than recreated per render/hook
  call.
  - `useSubscription` (`src/hooks/useSubscription.ts`) is a from-scratch polling implementation of
    contract event subscriptions (Soroban clients don't generate one) — keyed by
    `${contractId}:${topic}`, tracks paging cursors in a module-level object outside React state, and is
    not currently wired into any component (present for future use/reference).
- **Wallet integration**: `src/providers/WalletProvider.tsx` wraps `@creit.tech/stellar-wallets-kit`
  (`src/util/wallet.ts`). Because the kit has no way to query connection state directly, the provider
  round-trips wallet id/address/network through `localStorage` (`src/util/storage.ts`, a typed
  wrapper) and polls every second (`POLL_INTERVAL`) to keep React state in sync with the wallet
  extension. `useWallet()` is the consumer-facing hook; it throws if used outside the provider.
- **Pages/components**: single-page app (`src/pages/Home.tsx`) rendering a fixed grid of 20
  `ArtCard`s (`src/components/ArtCard.tsx`), one per token ID 0–19 — the collection size (20) and the
  art asset naming (`public/art/{00..19}-squares.png`) are not derived from contract state, they're
  hardcoded to match how `deploy_collection` was invoked. Each `ArtCard` independently queries
  owner/token URI and, when the gallery still owns the token, offers a purchase button that calls
  `squares_gallery.purchase_nft` and signs/sends via the connected wallet.
  CSS is CSS Modules (`*.module.css`) throughout.

### Environment variables

Frontend env vars must be prefixed `PUBLIC_` to be exposed to client code (Vite `envPrefix`). See
`.env.example` for the full set and per-network values (LOCAL/TESTNET/FUTURENET/PUBLIC); `.env.staging`
is checked in (unlike other `.env*` files) and used by `build:staging`/CI. `STELLAR_SCAFFOLD_ENV`
controls which `environments.toml` section the Stellar CLI/scaffold tooling builds against, independent
of the `PUBLIC_STELLAR_NETWORK` the frontend reads at runtime — keep these two in sync manually when
switching environments.

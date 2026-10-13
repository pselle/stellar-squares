# AGENTS.md

Guidance for AI coding agents working in this repository. `CLAUDE.md` imports this file.

## What this is

Stellar Squares is a generative-art NFT gallery dApp built with
[Stellar Scaffold](https://stellarscaffold.org) and the [Stellar Registry](https://rgstry.xyz). A
React/TypeScript frontend talks to a Soroban "gallery" contract that deploys NFT collections and sells
their tokens for XLM. Each collection is an instance of OpenZeppelin's `oz/nft-standard` Wasm from the
Registry.

Before changing Scaffold or Registry usage, read the upstream skills. They are the source of truth for
CLI commands and config, and they change faster than this file:

- Stellar Scaffold: https://raw.githubusercontent.com/stellar-scaffold/cli/main/skills/stellar-scaffold/SKILL.md
- Stellar Registry: https://raw.githubusercontent.com/stellar-registry/cli/main/skills/stellar-registry/SKILL.md

## Layout

```
contracts/squares-gallery/        the project's contract (contract.rs, nft.rs, test.rs)
contracts/nft-sequential-minting/ example NFT contract; only used to generate the app's NFT client
app/                              React + Vite frontend (its own npm workspace)
app-lib/                          shared wallet/network helpers, copied from the Scaffold template
app-lib/clients/                  GENERATED contract clients. Gitignored. Never edit.
e2e/                              Playwright smoke tests
scripts/registry.sh               publish / deploy / register / upgrade through the Registry
environments.toml                 networks, accounts and contracts per Scaffold environment
```

## Commands

Run from the repo root unless noted.

- `npm start`: the dev loop (`stellar scaffold watch --build-clients` plus Vite). Needs the local
  network and the one-time setup in the README's "Develop against a local network".
- `STELLAR_SCAFFOLD_ENV=staging stellar scaffold build --build-clients`: generate clients for the
  contracts deployed on testnet. Do this before linting, building or running e2e tests in a fresh clone.
- `npm run build:staging`: the command above plus the app build in staging mode. This is what CI deploys.
- `npm run lint`, `npm run format:check`, `npm run format`.
- `cargo test`: contract tests. `cargo test -p squares-gallery <name>` for one test.
- `npm run test:e2e`: Playwright against a Vite server in staging mode. Reads from testnet.

CI (`.github/workflows/build.yml`) runs, in order: Scaffold staging build with clients, lint, format
check, app build, `cargo test`, e2e tests. Run the same before opening a PR.

## Things that will trip you up

- **Contract builds contact the Registry.** `contracts/squares-gallery/src/nft.rs` uses
  `stellar_registry::import_contract_client!("oz/nft-standard@0.7.2")`, which runs `stellar registry
download` during `cargo build` unless the Wasm is already at
  `target/stellar/<network>/oz__nft_standard_0_7_2.wasm`. It needs the `stellar` and `stellar-registry`
  CLIs on `PATH`, plus a network and source account: either Stellar CLI defaults (`stellar network use
testnet`, `stellar keys use <name>`) or `STELLAR_NETWORK` and `STELLAR_ACCOUNT`. `<network>` is
  `STELLAR_NETWORK`, defaulting to `local`.
- **`app/.env` changes which Stellar config is used.** It sets `XDG_CONFIG_HOME=".config"`, so
  `stellar` commands run from `app/` (including `npm start`) use identities in `app/.config`, not your
  global ones. Commands run from the repo root use the global config.
- **Environment variables beat `.env` files in Vite.** A `PUBLIC_STELLAR_*` variable set in the shell
  or a workflow overrides `app/.env.staging`. This broke CI once.
- **The clients must match the network the app runs on.** `app-lib/clients/index.ts` has contract IDs
  for whichever `STELLAR_SCAFFOLD_ENV` was built last; the app connects using `PUBLIC_STELLAR_*`. After
  a local `npm start`, rebuild the staging clients before running e2e tests.
- **Import clients from the index.** Use `import { squaresGallery } from
"@stellar-scaffold/app-lib/clients"`. The one exception is `app/src/hooks/useNftCollection.ts`, which
  builds its own NFT client because the collection address is only known at runtime.
- **`app-lib/` is template code.** Prefer wrapping it in `app/` over editing it, so it stays easy to
  update from the Scaffold template.
- **`environments.toml` is the legacy Scaffold config.** This project moves to `scaffold.yml` version 2
  once a Scaffold CLI release supports it. Do not hand-migrate early.

## Contracts

- `squares-gallery` is owner-controlled. `deploy_collection` deploys the NFT Wasm (by `nft_wasm_hash`,
  set in the constructor) at an address derived from the collection symbol, mints the collection to the
  gallery, and records the price. `purchase_nft` moves XLM from the buyer and transfers the token.
  `withdraw` and `upgrade` are owner-only.
- `upgrade` replaces the code but does not rerun the constructor, so storage keys and constructor
  arguments must stay compatible across versions.
- The published version is `version` in `contracts/squares-gallery/Cargo.toml`. Bump it for every
  publish; the Registry rejects a version that is not higher than the last.

## Deployed state

- Testnet: the gallery is `unverified/squares-gallery` in the Registry. Look it up with
  `scripts/registry.sh testnet id`; do not hardcode the address.
- Mainnet: not deployed. The `production` IDs in `environments.toml` are placeholders.

Publishing, registering, deploying and upgrading write to a public network and cannot be undone. Use
`--dry-run` where the command supports it, and get the maintainer's go-ahead before running any of
them for real.

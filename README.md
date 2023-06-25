### Playground

TODO: - rewrite code targetting beta 3 testnet

TODO: - bridges

https://sway-playground.org/

### Browser wallet

https://wallet.fuel.network/docs/install/

### Install

https://github.com/FuelLabs/fuelup

```shell
rustup install stable
rustup update
rustup default stable

fuelup toolchain install latest

fuelup toolchain install beta-3
fuelup default beta-3

fuelup show
# switch
fuelup default latest
```

### Update

```shell
fuelup self update

# generate test
cargo generate --init fuellabs/sway templates/sway-test-rs --name counter

```

### New project

```shell
forc new counter_contract

# compile
forc build
# format
forc fmt

# node
fuel-core run --db-type in-memory
# deploy
forc deploy --unsigned


```

### Apps

-   [x] counter (storage)
-   [x] ownership (Identity type, msg_sender, configurable, Option, Error, imports)
-   [x] wallet (native assets, identity, access control, payable, output variables)
-   native token (native assets, asset id, payable, output variables)
-   [x] wrapped token (contract_id, msg_asset_id, mint, burn, transfer)
-   [x] nft (log, nested storage map, private funcs, constant)
-   [x] airdrop (sway-libs, events, multi abi, multi contracts, storage map, sha256, test events)
-   [x] call (multiple contracts, call, low level call, calling other contracts (Rust SDK), fn_selector!)
-   [x] multi-sig (multi token, vec on heap, multi abi, loop, storage vec, events, low_level_call, hash, recover sig)
-   [ ] otc (predicate)
-   [ ] bridge?
-   [ ] liquidity book - amm
-   uniswap v3 amm?
-   escrow
-   auction
-   queue (generics)
-   reentrancy guard?

### Topcis

-   no inheritance, no constructor, global memory, native assets, no for loop

-   deploy to testnet
-   sway lib
-   default values

-   program types overview

-   basic

    -   [x] variables (immutable, `mut`, type annotations)
    -   [ ] built-in
        -   [ ] primitive types (`u64`, `bool`, `str[]`, `b256`)
        -   [ ] compound type (tuple, struct, array)
    -   [ ] destructuring
    -   [ ] option and result
    -   [ ] blockchain
    -   [ ] functions
    -   [ ] structs
    -   [ ] tuples
    -   [ ] enums
    -   [ ] methods
    -   [ ] constants
    -   [ ] logging
    -   [ ] control flow

-   blockchain
    -   hashing and cryptography
    -   storage
    -   function purity
    -   identifier
    -   native assets
    -   access control
    -   calling contracts
-   advanced
    -   generics
    -   traits
    -   assembly
-   collections
    -   vector on heap
    -   vector and map
-   Testing (unit, integration)
-   Frontend
-   Deploy, node
-   wallet
-   inheritance?
-   assert, require, revert
-   program types (contract, library, scripts, predicates)

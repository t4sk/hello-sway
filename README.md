### Playground

https://sway-playground.org/

### Browser wallet

https://wallet.fuel.network/docs/install/

### Install

https://github.com/FuelLabs/fuelup

```shell
fuelup toolchain install latest

fuelup toolchain install beta-3
fuelup default beta-3
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
    # TODO: implement low level call for multisig
-   [ ] multi-sig (multi token, vec on heap, multi abi, loop, storage vec, events, low_level_call, hash, recover sig)
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

-   inheritance?
-   assert, require, revert
-   program types (contract, library, scripts, predicates)
-   basic
    -   variables
    -   built-in
    -   blockchain
    -   functions
    -   structs
    -   tuples
    -   enums
    -   methods
    -   logging
    -   control flow
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

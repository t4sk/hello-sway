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

-   counter (storage)
-   ownership (Identity type, msg_sender, configurable, Option, Error, imports)
-   wallet (native assets, identity, access control, payable)
-   token
-   bridge?
-   multi-sig (multi token)
-   escrow
-   amm
-   auction
-   nft
-   airdrop

### Topcis

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

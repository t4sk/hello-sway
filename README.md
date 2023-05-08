### Install

https://github.com/FuelLabs/fuelup

```shell
fuelup toolchain install latest
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

-   Program types

    -   contract, library, scripts, predicates

-   Basic
    -   variables
    -   built-in types
    -   blockchain types
    -   functions
    -   structs, tuples and enums
    -   methods and associated functions
    -   control flows
    -   comments and logging

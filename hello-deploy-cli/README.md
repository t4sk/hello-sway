# Deploy contract to Beta-3 Testnet

https://fuelbook.fuel.network/master/quickstart/smart-contract.html

# Install

```shell
# rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# fuel toolchain
curl --proto '=https' --tlsv1.2 -sSf https://install.fuel.network/fuelup-init.sh | sh

# update
fuelup self update

# install beta-3 toolchain
fuelup default beta-3

# list toolchain
fuelup show
```

# First contract

```shell
forc new my-contract
forc build
```

# Setup wallet

```shell
# Install wallet
fuelup toolchain install latest
forc-wallet --version

# New wallet
forc-wallet new

# Mnemonic
peace film ticket parade
heart globe story cannon
once slam abstract lens
angry clarify master bench
clerk city permit warrior
primary before demise inmate

# Import from existing mnemonic
forc-wallet import

# Create account
forc-wallet account new

fuel1swtt27kn8sfnc8r259lamr2pqlvc4s5xgurvxlk3m66eexywkutscmay0s

# List accounts
forc-wallet accounts
```

# Testnet faucet

https://faucet-beta-3.fuel.network/

https://fuellabs.github.io/block-explorer-v2/beta-3/#/

# Deploy

```shell
forc deploy --node-url beta-3.fuel.network/graphql --gas-price 1 --random-salt

# Contract id
0xa29359df98c43da3b8cd1b3d0de673565e6baa0ce485463516cde2289900e84f

# Get signature - open another terminal tab
ACCOUNT_INDEX=0
TX_ID=d67cc8c14fa25a27bc5fec08898451f7269e50af7c0e517885c2da948493067c
forc wallet sign --account 0 tx-id $TX_ID

# Copy and paste signature over to first terminal tab

# Transaction id
0xd67cc8c14fa25a27bc5fec08898451f7269e50af7c0e517885c2da948493067c
```

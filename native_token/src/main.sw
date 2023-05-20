contract;

use std::{
    context::{
        balance_of,
        msg_amount,
    },
    token::{
        burn,
        force_transfer_to_contract,
        mint,
        mint_to_address,
        mint_to_contract,
        transfer_to_address,
    },
};

abi MyToken {
    // Mint to this contract
    fn mint(amount: u64);
    // Burn from this contract
    fn burn(amount: u64);
    fn mint_to_address(amount: u64, to: Address);
    fn mint_to_contract(amount: u64, to: ContractId);
    fn transfer_to_address(amount: u64, token_id: ContractId, to: Address);
    fn force_transfer_to_contract(amount: u64, token_id: ContractId, contract_id: ContractId);
    // Deposit any token back to this contract
    fn deposit();
    fn get_balance_of_contract(contract_id: ContractId, token_id: ContractId) -> u64;
    // TODO: get balance of account?
}

impl MyToken for Contract {
    // Mint to this contract
    // Like ERC20.mint(amount, address(this))
    fn mint(amount: u64) {
        mint(amount);
    }

    // Burn from this contract
    // Like ERC20.burn(amount, address(this))
    fn burn(amount: u64) {
        burn(amount);
    }

    // Like ERC20.mint(to, amount)
    fn mint_to_address(amount: u64, to: Address) {
        // TODO: what happens if to is contact?
        mint_to_address(amount, to);
    }

    // Like ERC20.mint(to, amount)
    fn mint_to_contract(amount: u64, to: ContractId) {
        mint_to_contract(amount, to);
    }

    // Like ERC20(token).transfer(to, amount)
    fn transfer_to_address(amount: u64, token_id: ContractId, to: Address) {
        transfer_to_address(amount, token_id, to);
    }

    // Like ERC20(token).transfer(to, amount)
    fn force_transfer_to_contract(amount: u64, token_id: ContractId, contract_id: ContractId) {
        force_transfer_to_contract(amount, token_id, contract_id)
    }

    // Like ERC20(token).transferFrom(msg.sender, address(this), amount)
    fn deposit() {
        assert(msg_amount() > 0)
    }

    fn get_balance_of_contract(contract_id: ContractId, token_id: ContractId) -> u64 {
        balance_of(contract_id, token_id)
    }
}

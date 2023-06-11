contract;

use std::{bytes::Bytes, b512::B512, hash::{sha256}, ecr::{ec_recover_address}};

struct SignParams {
    contract_id: ContractId,
    z: u64,
}

abi MyContract {
    fn get_data_hash(data: Bytes) -> b256;

    fn get_msg_hash(x: u64, y: u64, params: SignParams, data: Bytes) -> b256;

    fn recover(sig: B512, x: u64, y: u64, params: SignParams, data: Bytes) -> Address;
}

impl MyContract for Contract {
    fn get_data_hash(data: Bytes) -> b256 {
        // https://github.com/FuelLabs/sway/issues/3809
        data.sha256()
    }

    fn get_msg_hash(x: u64, y: u64, params: SignParams, data: Bytes) -> b256 {
        sha256((x, y, params, data.sha256()))
    }

    fn recover(sig: B512, x: u64, y: u64, params: SignParams, data: Bytes) -> Address {
        let msg_hash = sha256((x, y, params, data.sha256()));
        let signer = ec_recover_address(sig, msg_hash).unwrap().value;
        Address::from(signer)
    }
}

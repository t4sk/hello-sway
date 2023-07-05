contract;

use std::{b512::B512, ecr::ec_recover_address, hash::sha256};

abi MyContract {
    fn get_hash(num: u64) -> b256;
    fn test_sig(sig: B512, hash: b256) -> Address;
}

impl MyContract for Contract {
    fn get_hash(num: u64) -> b256 {
        sha256(num)
    }

    fn test_sig(sig: B512, hash: b256) -> Address {
        let res = ec_recover_address(sig, hash);

        let addr: b256 = match res {
            Result::Ok(addr) => addr.value,
            _ => revert(0),
        };

        Address::from(addr)
    }
}

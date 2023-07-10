contract;

use std::{
    bytes::Bytes,
    constants::{
        BASE_ASSET_ID,
    },
    context::msg_amount,
    hash::sha256,
    low_level_call::{
        call_with_function_selector,
        CallParams,
    },
};

abi Caller {
    #[payable]
    fn test_call(contract_id: ContractId);

    #[payable]
    fn test_low_level_call(contract_id: ContractId, selector: Bytes, data: Bytes);
}

abi Receiver {
    #[payable]
    fn test_func(x: u64, y: u64) -> (u64, u64);
}

struct LogCallerEvent {
    x: u64,
    y: u64,
}

impl Caller for Contract {
    #[payable]
    fn test_call(contract_id: ContractId) {
        let receiver = abi(Receiver, contract_id.into());
        let (x, y) = receiver.test_func {
            // NOTE: can also specify gas
            asset_id: BASE_ASSET_ID.into(),
            coins: msg_amount(),
        }(1, 2);

        log(LogCallerEvent { x, y });
    }

    #[payable]
    fn test_low_level_call(contract_id: ContractId, selector: Bytes, data: Bytes) {
        call_with_function_selector(
            contract_id,
            selector, 
            data,
            false,
            CallParams {
                gas: 10000,
                coins: msg_amount(),
                asset_id: BASE_ASSET_ID,
            }
        );
    }
}

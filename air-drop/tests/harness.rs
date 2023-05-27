use fuels::{prelude::*, tx::ContractId, types::Bits256};

mod set_up;
mod tree;

use set_up::{set_up};

#[tokio::test]
async fn test_claim() {
    let (instance, id, wallets) = set_up().await;

    // size = 32, elements = 1 of type u8
    let root = Bits256([1u8; 32]);
    let num_leaves = 0;

    let call_params = CallParameters::default().set_amount(10);

    instance
        .methods()
        .init(root, num_leaves)
        .call_params(call_params)
        .unwrap()
        .call()
        .await
        .unwrap();

}

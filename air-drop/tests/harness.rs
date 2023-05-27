use fuels::{prelude::*, types::Bits256, types::Identity};

mod set_up;
mod tree;

use set_up::set_up;

#[tokio::test]
async fn test_claim() {
    let (instance, id, wallets) = set_up().await;

    let index = 1;
    let num_leaves = 4;
    let leaves: Vec<(Identity, u64)> = vec![
        (Identity::Address(wallets[0].address().into()), 100),
        (Identity::Address(wallets[1].address().into()), 200),
        (Identity::Address(wallets[2].address().into()), 300),
        (Identity::Address(wallets[3].address().into()), 400),
    ];

    let sender = wallets[index as usize].clone();
    let amount = leaves[index as usize].1;

    let (merkle_tree, merkle_root, merkle_leaf, proof) = tree::build(index, leaves);

    // size = 32, elements = 1 of type u8
    // let root = Bits256([1u8; 32]);

    let call_params = CallParameters::default().set_amount(1000);

    instance
        .methods()
        .init(merkle_root, num_leaves)
        .call_params(call_params)
        .unwrap()
        .call()
        .await
        .unwrap();

    instance
        .with_account(sender.clone())
        .unwrap()
        .methods()
        .claim(amount, index, proof)
        .append_variable_outputs(1)
        .call()
        .await
        .unwrap();
}

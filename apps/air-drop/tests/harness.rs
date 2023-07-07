use fuels::{prelude::*, types::Bits256, types::Identity};

mod set_up;
mod tree;

use set_up::{get_wallets, set_up, ClaimEvent};

#[tokio::test]
async fn test_claim() {
    let wallets = get_wallets().await;

    let asset_id = ContractId::new(BASE_ASSET_ID.try_into().unwrap());

    let index = 1;
    let num_leaves = 4;
    let leaves: Vec<(Identity, u64)> = vec![
        (Identity::Address(wallets[0].address().into()), 100),
        (Identity::Address(wallets[1].address().into()), 200),
        (Identity::Address(wallets[2].address().into()), 300),
        (Identity::Address(wallets[3].address().into()), 400),
    ];

    let receiver = wallets[index as usize].clone();
    let amount = leaves[index as usize].1;

    let (merkle_tree, merkle_root, merkle_leaf, proof) = tree::build(index, leaves);

    let (instance, id) = set_up(wallets[0].clone(), asset_id, merkle_root, num_leaves).await;

    instance
        .methods()
        .deposit()
        .call_params(CallParameters::default().set_amount(1000))
        .unwrap()
        .call()
        .await
        .unwrap();

    instance
        .with_account(receiver.clone())
        .unwrap()
        .methods()
        .claim(amount, index, proof)
        .append_variable_outputs(1)
        .call()
        .await
        .unwrap();

    let bal = receiver.get_asset_balance(&BASE_ASSET_ID).await.unwrap();
    println!("receiver balance {:?}", bal);
}

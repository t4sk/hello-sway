use fuels::{prelude::*, types::Bits256, types::Identity};

mod set_up;
mod tree;

use set_up::{set_up, InitEvent, ClaimEvent};

#[tokio::test]
async fn test_init() {
    let (instance, id, wallets) = set_up().await;

    let asset_id = ContractId::new(BASE_ASSET_ID.try_into().unwrap());

    // size = 32, elements = 1 of type u8
    let merkle_root = Bits256([1u8; 32]);
    let num_leaves = 4;

    let call_params = CallParameters::default().set_amount(1000);

    let res = instance
        .methods()
        .init(merkle_root, num_leaves)
        .call_params(call_params)
        .unwrap()
        .call()
        .await
        .unwrap();

    let logs = res.decode_logs_with_type::<InitEvent>().unwrap();
    let event = logs.get(0).unwrap();

    assert_eq!(
        *event,
        InitEvent {
            asset: asset_id,
            merkle_root,
            num_leaves
        }
    );
    
    assert_eq!(
        instance.methods().asset().call().await.unwrap().value.unwrap(),
        asset_id
    );
    assert_eq!(
        instance.methods().merkle_root().call().await.unwrap().value.unwrap(),
        merkle_root
    );
    assert_eq!(
        instance.methods().num_leaves().call().await.unwrap().value,
        num_leaves
    );
}

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

    let call_params = CallParameters::default().set_amount(1000);

    instance
        .methods()
        .init(merkle_root, num_leaves)
        .call_params(call_params)
        .unwrap()
        .call()
        .await
        .unwrap();

    // let bal = wallet.get_asset_balance(asset).await.unwrap()

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

use fuels::accounts::predicate::Predicate;
use fuels::{prelude::*, types::ContractId};

// Load abi from json
abigen!(Predicate(
    name = "MyPredicate",
    abi = "out/debug/my-predicate-abi.json"
));

async fn get_wallets() -> Vec<WalletUnlocked> {
    let wallets = launch_custom_provider_and_get_wallets(
        WalletsConfig::new(
            Some(5),             /* Single wallet */
            Some(1),             /* Single coin (UTXO) */
            Some(1_000_000_000), /* Amount per coin */
        ),
        None,
        None,
    )
    .await;

    wallets
}

#[tokio::test]
async fn test_predicate() {
    let wallets = get_wallets().await;

    let data = MyPredicateEncoder::encode_data(123, 123);
    let my_predicate = Predicate::load_from("out/debug/my-predicate.bin")
        .unwrap()
        .with_provider(wallets[0].try_provider().unwrap().clone())
        .with_data(data);

    // Check predicate balance.
    wallets[0]
        .transfer(
            my_predicate.address(),
            100,
            BASE_ASSET_ID,
            TxParameters::default(),
        )
        .await
        .unwrap();

    let bal = my_predicate
        .get_asset_balance(&BASE_ASSET_ID)
        .await
        .unwrap();
    println!("predicate balance: {:?}", bal);

    // Transfer asset owned by predicate
    my_predicate
        .transfer(
            wallets[1].address(),
            100,
            BASE_ASSET_ID,
            TxParameters::default(),
        )
        .await
        .unwrap();

    let bal = my_predicate
        .get_asset_balance(&BASE_ASSET_ID)
        .await
        .unwrap();
    println!("predicate balance: {:?}", bal);

    let bal = wallets[1].get_asset_balance(&BASE_ASSET_ID).await.unwrap();
    println!("wallet 1 balance: {:?}", bal);
}

use fuels::{prelude::*, types::ContractId};
use fuels::accounts::predicate::Predicate;

// Load abi from json
abigen!(Predicate(
    name = "MyPredicate",
    abi = "out/debug/otc-abi.json"
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
async fn can_get_contract_id() {
    let wallets = get_wallets().await;

    let my_predicate = Predicate::load_from("out/debug/otc.bin").unwrap();

    // Now you have an instance of your contract you can use to test each function
}

use fuels::{prelude::*, tx::ContractId};

// Load abi from json
abigen!(Contract(
    name = "AirDrop",
    abi = "out/debug/air-drop-abi.json"
));

pub(crate) async fn set_up() -> (AirDrop<WalletUnlocked>, ContractId, Vec<WalletUnlocked>) {
    // Launch a local network and deploy the contract
    let mut wallets = launch_custom_provider_and_get_wallets(
        WalletsConfig::new(
            Some(4),
            Some(1),             /* Single coin (UTXO) */
            Some(1_000_000_000), /* Amount per coin */
        ),
        None,
        None,
    )
    .await;
    let wallet = &wallets[0];

    let id = Contract::deploy(
        "./out/debug/air-drop.bin",
        wallet,
        DeployConfiguration::default(),
    )
    .await
    .unwrap();

    let instance = AirDrop::new(id.clone(), wallet.clone());

    (instance, id.into(), wallets)
}

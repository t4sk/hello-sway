use fuels::{
    prelude::*,
    types::{Bits256, ContractId},
};

// Load abi from json
abigen!(Contract(
    name = "AirDrop",
    abi = "out/debug/air-drop-abi.json"
));

pub(crate) async fn get_wallets() -> Vec<WalletUnlocked> {
    let wallets = launch_custom_provider_and_get_wallets(
        WalletsConfig::new(
            Some(4),             /* Single wallet */
            Some(1),             /* Single coin (UTXO) */
            Some(1_000_000_000), /* Amount per coin */
        ),
        None,
        None,
    )
    .await;

    return wallets;
}

pub(crate) async fn set_up(
    wallet: WalletUnlocked,
    asset_id: ContractId,
    merkle_root: Bits256,
    num_leaves: u64,
) -> (AirDrop<WalletUnlocked>, ContractId) {
    let config = AirDropConfigurables::new()
        .set_ASSET(asset_id)
        .set_MERKLE_ROOT(merkle_root)
        .set_NUM_LEAVES(num_leaves);

    let id = Contract::load_from(
        "./out/debug/air-drop.bin",
        LoadConfiguration::default().set_configurables(config),
    )
    .unwrap()
    .deploy(&wallet, TxParameters::default())
    .await
    .unwrap();

    let instance = AirDrop::new(id.clone(), wallet.clone());

    (instance, id.into())
}

use fuels::{
    accounts::wallet::WalletUnlocked,
    prelude::*,
    types::{ContractId, Identity},
};

// Load abi from json
abigen!(Contract(
    name = "MyContract",
    abi = "out/debug/test-token-transfer-abi.json"
));

async fn get_contract_instance() -> (MyContract<WalletUnlocked>, ContractId, Vec<WalletUnlocked>) {
    // Launch a local network and deploy the contract
    let mut wallets = launch_custom_provider_and_get_wallets(
        WalletsConfig::new(
            Some(5),             /* Single wallet */
            Some(1),             /* Single coin (UTXO) */
            Some(1_000_000_000), /* Amount per coin */
        ),
        None,
        None,
    )
    .await;
    let wallet = wallets.pop().unwrap();

    let id = Contract::load_from(
        "./out/debug/test-token-transfer.bin",
        LoadConfiguration::default(),
    )
    .unwrap()
    .deploy(&wallet, TxParameters::default())
    .await
    .unwrap();

    let instance = MyContract::new(id.clone(), wallet);

    (instance, id.into(), wallets)
}

#[tokio::test]
async fn test_transfer() {
    let (inst, inst_id, wallets) = get_contract_instance().await;

    // deposit
    let call_params = CallParameters::default()
        .set_amount(100)
        .set_asset_id(BASE_ASSET_ID);

    inst.methods()
        .deposit()
        .call_params(call_params)
        .unwrap()
        .call()
        .await
        .unwrap();

    let to_addr = Address::from(wallets[1].address());
    let to = Identity::Address(to_addr);

    let bal = wallets[1].get_asset_balance(&BASE_ASSET_ID).await.unwrap();
    println!("wallet balance before {:?}", bal);

    let res = inst
        .methods()
        .transfer(10, to)
        .append_variable_outputs(1)
        .call()
        .await
        .unwrap();

    let bal = wallets[1].get_asset_balance(&BASE_ASSET_ID).await.unwrap();
    println!("wallet balance after {:?}", bal);

    // let res = inst
    //     .methods()
    //     .get_balance_of_contract()
    //     .call()
    //     .await
    //     .unwrap();

    // println!("contract balance {:?}", res.value);
}

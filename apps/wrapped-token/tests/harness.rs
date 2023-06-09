use fuels::{prelude::*, tx::ContractId, types::AssetId};

// Load abi from json
abigen!(Contract(
    name = "WrappedToken",
    abi = "out/debug/wrapped-token-abi.json"
));

async fn get_contract_instance() -> (
    WrappedToken<WalletUnlocked>,
    ContractId,
    Vec<WalletUnlocked>,
) {
    // Launch a local network and deploy the contract
    let mut wallets = launch_custom_provider_and_get_wallets(
        WalletsConfig::new(
            Some(2),             /* Single wallet */
            Some(1),             /* Single coin (UTXO) */
            Some(1_000_000_000), /* Amount per coin */
        ),
        None,
        None,
    )
    .await;
    let wallet = &wallets[0];

    let id = Contract::load_from(
        "./out/debug/wrapped-token.bin",
        LoadConfiguration::default(),
    )
    .unwrap()
    .deploy(wallet, TxParameters::default())
    .await
    .unwrap();

    let instance = WrappedToken::new(id.clone(), wallet.clone());

    (instance, id.into(), wallets)
}

#[tokio::test]
async fn test_deposit_and_withdraw_to_address() {
    let (instance, contract_id, wallets) = get_contract_instance().await;
    let asset_id = AssetId::new(contract_id.into());

    // Deposit base asset
    instance
        .methods()
        .deposit()
        .call_params(CallParameters::default().set_amount(10))
        .unwrap()
        .append_variable_outputs(1)
        .call()
        .await
        .unwrap();

    // Check wrapped token balance of wallet[0]
    let bal = wallets[0].get_asset_balance(&asset_id).await.unwrap();
    assert_eq!(bal, 10);

    // Withdraw
    instance
        .methods()
        .withdraw()
        .call_params(
            CallParameters::default()
                .set_amount(1)
                .set_asset_id(asset_id),
        )
        .unwrap()
        .append_variable_outputs(1)
        .call()
        .await
        .unwrap();

    let bal = wallets[0].get_asset_balance(&asset_id).await.unwrap();
    assert_eq!(bal, 9);
}

#[tokio::test]
#[should_panic(expected = "not base asset")]
async fn test_deposit_not_base_asset() {
    let (instance, contract_id, wallets) = get_contract_instance().await;

    let asset_id = AssetId::new(contract_id.into());
    // Deposit base asset to obtain wrapped asset
    instance
        .methods()
        .deposit()
        .call_params(CallParameters::default().set_amount(10))
        .unwrap()
        .append_variable_outputs(1)
        .call()
        .await
        .unwrap();

    // Try to deposit wrapped asset - should fail
    instance
        .methods()
        .deposit()
        .call_params(
            CallParameters::default()
                .set_amount(10)
                .set_asset_id(asset_id),
        )
        .unwrap()
        .append_variable_outputs(1)
        .call()
        .await
        .unwrap();
}

#[tokio::test]
#[should_panic(expected = "msg amount = 0")]
async fn test_deposit_zero_amount() {
    let (instance, contract_id, wallets) = get_contract_instance().await;

    let asset_id = AssetId::new(contract_id.into());

    instance
        .methods()
        .deposit()
        .call_params(CallParameters::default().set_amount(0))
        .unwrap()
        .append_variable_outputs(1)
        .call()
        .await
        .unwrap();
}

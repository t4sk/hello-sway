use fuels::{prelude::*, types::{Identity, ContractId}};

// Load abi from json
abigen!(Contract(
    name = "MyWallet",
    abi = "out/debug/wallet-abi.json"
));

async fn get_contract_instance() -> (MyWallet<WalletUnlocked>, ContractId, Vec<WalletUnlocked>) {
    // Launch a local network and deploy the contract
    let mut wallets = launch_custom_provider_and_get_wallets(
        WalletsConfig::new(
            Some(2),
            Some(1),             /* Single coin (UTXO) */
            Some(1_000_000_000), /* Amount per coin */
        ),
        None,
        None,
    )
    .await;
    let wallet = &wallets[0];

    let id = Contract::load_from(
        "./out/debug/wallet.bin",
        LoadConfiguration::default(),
    ).unwrap()
    .deploy(wallet, TxParameters::default())
    .await
    .unwrap();

    let instance = MyWallet::new(id.clone(), wallet.clone());

    (instance, id.into(), wallets)
}

#[tokio::test]
async fn test_init() {
    let (instance, contract_id, wallets) = get_contract_instance().await;

    instance.methods().init().call().await.unwrap();

    let res = instance.methods().owner().call().await.unwrap();
    let addr = Address::from(wallets[0].address());
    let wallet_id = Identity::Address(addr);

    assert_eq!(res.value.unwrap(), wallet_id);
}

#[tokio::test]
async fn test_deposit() {
    let (instance, contract_id, wallets) = get_contract_instance().await;

    let bal = wallets[0].get_asset_balance(&BASE_ASSET_ID).await.unwrap();
    println!("{:?}", bal);

    let call_params = CallParameters::default().set_amount(10);

    instance
        .methods()
        .deposit()
        .call_params(call_params)
        .unwrap()
        .call()
        .await
        .unwrap();

    let res = instance.methods().balance().call().await.unwrap();
    assert_eq!(res.value, 10);
}

#[tokio::test]
async fn test_send() {
    let (instance, contract_id, wallets) = get_contract_instance().await;

    instance.methods().init().call().await.unwrap();

    let call_params = CallParameters::default().set_amount(10);

    instance
        .methods()
        .deposit()
        .call_params(call_params)
        .unwrap()
        .call()
        .await
        .unwrap();

    instance
        .methods()
        .send(Address::from(wallets[1].address()), 1)
        .append_variable_outputs(1)
        .call()
        .await
        .unwrap();

    let bal = wallets[1].get_asset_balance(&BASE_ASSET_ID).await.unwrap();
    assert_eq!(bal, 1000000001);
}

#[tokio::test]
#[should_panic(expected = "UnauthorizedError")]
async fn test_send_unauthorized() {
    let (instance, contract_id, wallets) = get_contract_instance().await;

    instance.methods().init().call().await.unwrap();

    let call_params = CallParameters::default().set_amount(10);

    instance
        .methods()
        .deposit()
        .call_params(call_params)
        .unwrap()
        .call()
        .await
        .unwrap();

    instance
        .with_account(wallets[1].clone())
        .unwrap()
        .methods()
        .send(Address::from(wallets[1].address()), 1)
        .append_variable_outputs(1)
        .call()
        .await
        .unwrap();
}

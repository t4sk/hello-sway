use fuels::{
    prelude::*,
    types::{ContractId, Identity},
};

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

    let id = Contract::load_from("./out/debug/wallet.bin", LoadConfiguration::default())
        .unwrap()
        .deploy(wallet, TxParameters::default())
        .await
        .unwrap();

    let instance = MyWallet::new(id.clone(), wallet.clone());

    (instance, id.into(), wallets)
}

async fn deposit(instance: &MyWallet<WalletUnlocked>, amount: u64) {
    instance
        .methods()
        .deposit()
        .call_params(CallParameters::default().set_amount(amount))
        .unwrap()
        .call()
        .await
        .unwrap();
}

async fn withdraw(
    instance: &MyWallet<WalletUnlocked>,
    from: WalletUnlocked,
    to: &WalletUnlocked,
    amount: u64,
) {
    let to_id = Identity::Address(Address::from(to.address()));

    // NOTE: - variable outputs
    instance
        .with_account(from.clone())
        .unwrap()
        .methods()
        .send(to_id, 1)
        .append_variable_outputs(1)
        .call()
        .await
        .unwrap();
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

    instance.methods().init().call().await.unwrap();

    deposit(&instance, 10).await;

    let provider = wallets[0].try_provider().unwrap();
    let bal = provider
        .get_contract_asset_balance(&Bech32ContractId::from(contract_id.clone()), BASE_ASSET_ID)
        .await
        .unwrap();
    assert_eq!(bal, 10);
}

#[tokio::test]
async fn test_send() {
    let (instance, contract_id, wallets) = get_contract_instance().await;

    instance.methods().init().call().await.unwrap();

    deposit(&instance, 10).await;
    withdraw(&instance, wallets[0].clone(), &wallets[1], 1).await;

    let bal = wallets[1].get_asset_balance(&BASE_ASSET_ID).await.unwrap();
    assert_eq!(bal, 1000000001);
}

#[tokio::test]
#[should_panic(expected = "NotAuthorized")]
async fn test_send_unauthorized() {
    let (instance, contract_id, wallets) = get_contract_instance().await;

    instance.methods().init().call().await.unwrap();

    let call_params = CallParameters::default().set_amount(10);

    deposit(&instance, 10).await;
    withdraw(&instance, wallets[1].clone(), &wallets[1], 1).await;
}

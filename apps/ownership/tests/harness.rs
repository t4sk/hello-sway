use fuels::{prelude::*, tx::ContractId, types::Identity};

// Load abi from json
abigen!(Contract(
    name = "MyContract",
    abi = "out/debug/ownership-abi.json"
));

async fn get_contract_instance() -> (MyContract<WalletUnlocked>, ContractId, Vec<WalletUnlocked>) {
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

    let id = Contract::deploy(
        "./out/debug/ownership.bin",
        wallet,
        DeployConfiguration::default(),
    )
    .await
    .unwrap();

    let instance = MyContract::new(id.clone(), wallet.clone());

    (instance, id.into(), wallets)
}

#[tokio::test]
async fn test_init() {
    let (instance, contract_id, wallets) = get_contract_instance().await;

    instance.methods().init().call().await.unwrap();

    let res = instance.methods().owner().call().await.unwrap();
    let addr = Address::from(wallets[0].address());
    let wallet_id = Identity::Address(addr);

    // println!("{:?}", res.value);
    // println!("{:?}", wallet);

    assert_eq!(res.value.unwrap(), wallet_id);
}

#[tokio::test]
async fn test_set_owner() {
    let (instance, contract_id, wallets) = get_contract_instance().await;

    instance.methods().init().call().await.unwrap();

    let addr = Address::from(wallets[1].address());
    let wallet_id = Identity::Address(addr);

    instance
        .methods()
        .set_owner(wallet_id.clone())
        .call()
        .await
        .unwrap();

    let res = instance.methods().owner().call().await.unwrap();
    assert_eq!(res.value.unwrap(), wallet_id);
}

#[tokio::test]
#[should_panic(expected = "UnauthorizedError")]
async fn test_set_owner_unauthorized() {
    let (instance, contract_id, wallets) = get_contract_instance().await;

    instance.methods().init().call().await.unwrap();

    let addr = Address::from(wallets[1].address());
    let wallet_id = Identity::Address(addr);

    instance
        .with_account(wallets[1].clone())
        .unwrap()
        .methods()
        .set_owner(wallet_id.clone())
        .call()
        .await
        .unwrap();
}

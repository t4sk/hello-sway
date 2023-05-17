use fuels::{prelude::*, tx::ContractId};

// Load abi from json
abigen!(Contract(
    name = "MyContract",
    abi = "out/debug/ownership-abi.json"
));

async fn get_contract_instance() -> (MyContract<WalletUnlocked>, ContractId) {
    // Launch a local network and deploy the contract
    let mut wallets = launch_custom_provider_and_get_wallets(
        WalletsConfig::new(
            Some(1),             /* Single wallet */
            Some(1),             /* Single coin (UTXO) */
            Some(1_000_000_000), /* Amount per coin */
        ),
        None,
        None,
    )
    .await;
    let wallet = wallets.pop().unwrap();

    let id = Contract::deploy(
        "./out/debug/ownership.bin",
        &wallet,
        DeployConfiguration::default(),
    )
    .await
    .unwrap();

    let instance = MyContract::new(id.clone(), wallet);

    (instance, id.into())
}

// #[tokio::test]
// async fn can_get_owner() {
//     let (instance, id) = get_contract_instance().await;

//     let res = instance.methods().owner().call().await.unwrap();
//     println!("{:?}", res);
// }

#[tokio::test]
async fn test_init() {
    let (instance, id) = get_contract_instance().await;

    instance.methods().init().call().await.unwrap();

    let res = instance.methods().owner().call().await.unwrap();

    println!("{:?}", res);
}
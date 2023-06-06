use fuels::{
    prelude::*,
    types::{ContractId, Identity},
};

// Load abi from json
abigen!(
    Contract(
        name = "MultiSigWallet",
        abi = "multi-sig-wallet/out/debug/multi-sig-wallet-abi.json"
    ),
    Contract(
        name = "TestContract",
        abi = "test-contract/out/debug/test-contract-abi.json"
    ),
);

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

    return wallets;
}

async fn get_multi_sig_instance(
    wallet: WalletUnlocked,
) -> (MultiSigWallet<WalletUnlocked>, ContractId) {
    let id = Contract::load_from(
        "./multi-sig-wallet/out/debug/multi-sig-wallet.bin",
        LoadConfiguration::default(),
    )
    .unwrap()
    .deploy(&wallet, TxParameters::default())
    .await
    .unwrap();

    let instance = MultiSigWallet::new(id.clone(), wallet);

    (instance, id.into())
}

async fn get_test_contract_instance(
    wallet: WalletUnlocked,
) -> (TestContract<WalletUnlocked>, ContractId) {
    let id = Contract::load_from(
        "./test-contract/out/debug/test-contract.bin",
        LoadConfiguration::default(),
    )
    .unwrap()
    .deploy(&wallet, TxParameters::default())
    .await
    .unwrap();

    let instance = TestContract::new(id.clone(), wallet);

    (instance, id.into())
}

#[tokio::test]
async fn test_init() {
    let wallets = get_wallets().await;
    let (multi_sig, multi_sig_id) = get_multi_sig_instance(wallets[0].clone()).await;

    let ids = vec![wallets[0].clone(), wallets[1].clone(), wallets[2].clone()]
        .iter()
        .map(|w| Identity::Address(Address::from(w.address())))
        .collect::<Vec<Identity>>();

    multi_sig.methods().init(ids.clone()).call().await.unwrap();

    let res = multi_sig.methods().owners().call().await.unwrap();
    assert_eq!(res.value.len(), 3);
    assert_eq!(res.value[0], ids[0]);
    assert_eq!(res.value[1], ids[1]);
    assert_eq!(res.value[2], ids[2]);
}

// TODO: test deposit
// TODO: test withdraw
// TODO: test execute with signatures

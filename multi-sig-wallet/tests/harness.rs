use fuels::{
    core::{calldata, fn_selector},
    prelude::*,
    types::{ContractId, Identity},
};
use sha2::{Digest, Sha256};

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
#[tokio::test]
async fn test_execute() {
    let wallets = get_wallets().await;
    let (multi_sig, multi_sig_id) = get_multi_sig_instance(wallets[0].clone()).await;
    let (test_contract, test_contract_id) = get_test_contract_instance(wallets[0].clone()).await;

    let nonce: u64 = 0;
    let call_params = CallParams {
        coins: 0,
        asset_id: ContractId::new(*BASE_ASSET_ID),
        gas:100_000
    };
    let data = calldata!();

    let execute_params = ExecuteParams {
        contract_id: test_contract_id,
        fn_selector: Bytes(fn_selector!(test_function())),
        data: Bytes(data),
        single_value_type_arg: true,
        call_params: call_params
    };

    let mut hasher = Sha256::new();
    hasher.update(multi_sig_id);
    hasher.update(calldata!(execute_params));
    hasher.update(nonce.to_be_bytes());
    let tx_hash: [u8; 32] = hasher.finalize().try_into().unwrap();

    println!("{:#?}", tx_hash);

}
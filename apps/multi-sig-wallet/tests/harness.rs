use fuels::{
    accounts::wallet::WalletUnlocked,
    accounts::fuel_crypto,
    accounts::fuel_crypto::fuel_types::Bytes64,
    // core::{calldata, fn_selector},
    // core::{abi_encoder::ABIEncoder, function_selector},
    prelude::*,
    types::{
        transaction_builders::{ScriptTransactionBuilder, TransactionBuilder},
        Bits256, ContractId, Identity, B512,
    },
};
use sha2::{Digest, Sha256};

// Copied from 0.42 fuels::core::fn_selector
macro_rules! fn_selector {
    ( $fn_name: ident ( $($fn_arg: ty),* )  ) => {
         fuels::core::function_selector::resolve_fn_selector(stringify!($fn_name), &[$( <$fn_arg as fuels::types::traits::Parameterize>::param_type() ),*]).to_vec()
    }
}

// Copied from 0.42 fuels::core::calldata
macro_rules! calldata {
    ( $($arg: expr),* ) => {
        fuels::core::abi_encoder::ABIEncoder::encode(&[$(fuels::types::traits::Tokenizable::into_token($arg)),*]).unwrap().resolve(0)
    }
}

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

    let instance = MultiSigWallet::new(id.clone(), wallet.clone());

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

// #[tokio::test]
// async fn test_init() {
//     let wallets = get_wallets().await;
//     let (multi_sig, multi_sig_id) = get_multi_sig_instance(wallets[0].clone()).await;

//     let ids = vec![wallets[0].clone(), wallets[1].clone(), wallets[2].clone()]
//         .iter()
//         .map(|w| Identity::Address(Address::from(w.address())))
//         .collect::<Vec<Identity>>();

//     multi_sig.methods().init(ids.clone()).call().await.unwrap();

//     let res = multi_sig.methods().owners().call().await.unwrap();
//     assert_eq!(res.value.len(), 3);
//     assert_eq!(res.value[0], ids[0]);
//     assert_eq!(res.value[1], ids[1]);
//     assert_eq!(res.value[2], ids[2]);
// }

#[tokio::test]
async fn test_execute() {
    let wallets = get_wallets().await;
    let (multi_sig, multi_sig_id) = get_multi_sig_instance(wallets[0].clone()).await;
    let (test_contract, test_contract_id) = get_test_contract_instance(wallets[0].clone()).await;

    // Init
    let ids = vec![wallets[0].clone(), wallets[1].clone(), wallets[2].clone()]
        .iter()
        .map(|w| Identity::Address(Address::from(w.address())))
        .collect::<Vec<Identity>>();

    multi_sig.methods().init(ids.clone()).call().await.unwrap();

    // Prepare params
    let nonce: u64 = 0;
    let call_params = CallParams {
        coins: 0,
        asset_id: ContractId::new(*BASE_ASSET_ID),
        gas: 100_000,
    };
    let selector = Bytes(fn_selector!(inc()));
    let data = Bytes(vec![]);

    let params = ExecuteParams {
        contract_id: test_contract_id,
        fn_selector: selector.clone(),
        data: data.clone(),
        single_value_type_arg: true,
        call_params: call_params,
    };

    // Hash fn selector
    let mut hasher = Sha256::new();
    hasher.update(selector.0.clone());
    let fn_selector_hash: [u8; 32] = hasher.finalize().try_into().unwrap();

    // Hash data
    let mut hasher = Sha256::new();
    hasher.update(data.0.clone());
    let data_hash: [u8; 32] = hasher.finalize().try_into().unwrap();

    let data_bytes = calldata!((
        multi_sig_id,
        params.clone().contract_id,
        Bits256(fn_selector_hash.clone()),
        Bits256(data_hash.clone()),
        params.clone().single_value_type_arg,
        params.clone().call_params,
        nonce
    ));

    let msg_hash = fuel_crypto::Message::new(data_bytes.clone());

    // Get tx hash
    let res = multi_sig
        .methods()
        .get_execute_tx_hash(params.clone(), nonce)
        .call()
        .await
        .unwrap();

    let tx_hash = res.value;

    println!("msg hash: {:?}", msg_hash);
    println!(
        "tx hash:  {:?}",
        unsafe {
            fuel_crypto::Message::from_bytes_unchecked(tx_hash.0)
        }
    );

    // Sort wallets by address
    let mut walls = [wallets[0].clone(), wallets[1].clone()];
    walls.sort_by(|a, b| a.address().hash().cmp(&b.address().hash()));

    // Sign
    let mut sigs: Vec<B512> = vec![];

    for i in 0..2 {
        let sig = walls[i].sign_message(&data_bytes.clone()).await.unwrap();
        // Convert Signature into B512
        let sig_bytes = Bytes64::from(sig);
        let b512 = B512::from((
            Bits256(sig_bytes[..32].try_into().unwrap()),
            Bits256(sig_bytes[32..].try_into().unwrap()),
        ));
        sigs.push(b512);

        let msg_hash = fuel_crypto::Message::new(data_bytes.clone());
        let rec = sig.recover(&msg_hash).unwrap();
        println!("wallet    {:?}", walls[i].clone().address().hash());
        println!("recovered {:?}", rec.hash());
    }

    // Check signers
    let res = multi_sig
        .methods()
        .get_signers(params.clone(), 0, sigs.clone())
        .call()
        .await
        .unwrap();

    println!("signers {:#?}", res.value);

    // Execute
    multi_sig
        .methods()
        .execute(params.clone(), sigs.clone())
        .set_contracts(&[&test_contract])
        .call()
        .await
        .unwrap();

    // Check test contract
    let res = test_contract.methods().get_count().call().await.unwrap();

    println!("test contract count {:#?}", res.value);
}

#[tokio::test]
async fn test_transfer() {
    let wallets = get_wallets().await;
    let (multi_sig, multi_sig_id) = get_multi_sig_instance(wallets[0].clone()).await;

    // Init
    let ids = vec![wallets[0].clone(), wallets[1].clone(), wallets[2].clone()]
        .iter()
        .map(|w| Identity::Address(Address::from(w.address())))
        .collect::<Vec<Identity>>();

    multi_sig.methods().init(ids.clone()).call().await.unwrap();

    // Send base asset to wallet
    let bals = multi_sig.get_balances().await.unwrap();
    println!("multi sig balances: {:?}", bals);

    let call_params = CallParameters::default().set_amount(100);

    multi_sig
        .with_account(wallets[0].clone())
        .unwrap()
        .methods()
        .deposit()
        .call_params(call_params)
        .unwrap()
        .call()
        .await
        .unwrap();

    let bals = multi_sig.get_balances().await.unwrap();
    println!("multi sig balances: {:?}", bals);

    // Prepare transfer data
    let params = TransferParams {
        to: Identity::Address(Address::from(wallets[1].address())),
        asset_id: ContractId::new(*BASE_ASSET_ID),
        amount: 1
    };

    let nonce: u64 = 0;
    let data_bytes = calldata!((
        multi_sig_id,
        params.clone(),
        nonce
    ));

    // Sort wallets by address
    let mut walls = [wallets[0].clone(), wallets[1].clone()];
    walls.sort_by(|a, b| a.address().hash().cmp(&b.address().hash()));

    // Sign
    let msg_hash = fuel_crypto::Message::new(data_bytes.clone());
    let mut sigs: Vec<B512> = vec![];

    for i in 0..2 {
        let sig = walls[i].sign_message(&data_bytes.clone()).await.unwrap();
        // Convert Signature into B512
        let sig_bytes = Bytes64::from(sig);
        let b512 = B512::from((
            Bits256(sig_bytes[..32].try_into().unwrap()),
            Bits256(sig_bytes[32..].try_into().unwrap()),
        ));
        sigs.push(b512);
    }

    // Withdraw from wallet
    println!("wallet {:?}", wallets[1].address());
    println!("param {:?}", params.clone().to);

    let res = multi_sig
        .methods()
        .transfer(params.clone(), sigs.clone())
        .append_variable_outputs(1)
        .call()
        .await
        .unwrap();

    let bal = wallets[1].get_asset_balance(&BASE_ASSET_ID).await.unwrap();
    println!("WALLET BALANCE {:?}", bal);
}

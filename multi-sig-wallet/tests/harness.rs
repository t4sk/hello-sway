use fuels::{
    core::{calldata, fn_selector, abi_encoder},
    prelude::*,
    types::{ContractId, Identity, Bits256, B512},
    accounts::fuel_crypto,
    accounts::fuel_crypto::fuel_types::{Bytes64},
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
unsafe fn any_as_u8_slice<T: Sized>(p: &T) -> &[u8] {
    ::core::slice::from_raw_parts(
        (p as *const T) as *const u8,
        ::core::mem::size_of::<T>(),
    )
}

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
        gas:100_000
    };
    let selector = Bytes(fn_selector!(test_function()));
    let data = Bytes(calldata!());

    let execute_params = ExecuteParams {
        contract_id: test_contract_id,
        fn_selector: selector,
        data,
        single_value_type_arg: true,
        call_params: call_params
    };

    // Get tx hash
    let res = multi_sig.methods().get_execute_tx_hash(
        execute_params.clone(),
        nonce
    ).call().await.unwrap();

    let tx_hash = res.value;

    // Sort wallets by address
    let mut walls = [
        wallets[0].clone(),
        wallets[1].clone(),
    ];
    walls.sort_by(|a, b| a.address().hash().cmp(&b.address().hash()));

    // Sign
    let mut sigs: Vec<B512> = vec![];

    for i in 0..2 {
        let sig = walls[i].sign_message(tx_hash.0).await.unwrap();
        // Convert Signature into B512
        let sig_bytes = Bytes64::from(sig);
        let b512 = B512::from((
            Bits256(sig_bytes[..32].try_into().unwrap()),
            Bits256(sig_bytes[32..].try_into().unwrap()),
        ));
        sigs.push(b512);
    }
    
    // let sig: u64 = wallets[0].sign_message(tx_hash.0).await.unwrap();
    // let msg = fuel_crypto::Message::new(tx_hash.0);
    // let rec = sig.recover(&msg).unwrap();

    // println!("message {:?}", msg);
    // println!("wallet {:?}", wallets[0].clone().address().hash());
    // println!("recovered {:?}", rec.hash());

    // println!("{:#?}", wallets[0]);

    // Execute
    let res = multi_sig.methods().execute(
        execute_params,
        sigs
    ).call().await.unwrap();

    println!("{:#?}", res);
    

    // let mut hasher = Sha256::new();

    // let c_data = calldata!(
    //     test_contract_id,
    //     // selector.clone()
    //     // data.clone(),
    //     // call_params.clone() 
    // );

    // hasher.update(multi_sig_id);

    // hasher.update(test_contract_id);
    // // hasher.update(c_data);

    // hasher.update(test_contract_id);
    // hasher.update([0, 0, 0, 0, 0, 0, 0, 1]);
    // hasher.update(selector.clone().0);
    // hasher.update(calldata!(data.clone()));
    // hasher.update([0, 0, 0, 0, 0, 0, 0, 1]);
    // hasher.update(call_params.coins.to_be_bytes());
    // hasher.update(call_params.asset_id);
    // hasher.update(call_params.gas.to_be_bytes());
    // // transfer params
    // hasher.update(multi_sig_id);
    // // address enum
    // match transfer_params.to {
    //     // Encoding enum type + identity data
    //     Identity::Address(identity) => {
    //         hasher.update([0, 0, 0, 0, 0, 0, 0, 0]);
    //         hasher.update(*identity);
    //     }
    //     Identity::ContractId(identity) => {
    //         hasher.update([0, 0, 0, 0, 0, 0, 0, 1]);
    //         hasher.update(*identity);
    //     }
    // }
    // hasher.update(&transfer_params.asset_id);
    // hasher.update(transfer_params.amount.to_be_bytes());
    // hasher.update(nonce.to_be_bytes());

    // let tx_hash: [u8; 32] = hasher.finalize().try_into().unwrap();
    // println!("{:?}", tx_hash);
}
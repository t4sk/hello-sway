use fuels::{
    core::{calldata, fn_selector},
    prelude::*,
    types::ContractId,
};

// Load abi from json
abigen!(
    Contract(
        name = "Receiver",
        abi = "receiver/out/debug/receiver-abi.json"
    ),
    Contract(name = "Caller", abi = "caller/out/debug/caller-abi.json")
);

async fn get_wallets() -> Vec<WalletUnlocked> {
    // Launch a local network and deploy the contract
    let wallets = launch_custom_provider_and_get_wallets(
        WalletsConfig::new(
            Some(2),             /* Single wallet */
            Some(1),             /* Single coin (UTXO) */
            Some(1_000_000_000), /* Amount per coin */
        ),
        None,
        None,
    )
    .await;

    return wallets;
}

async fn get_receiver_instance(wallet: WalletUnlocked) -> (Receiver<WalletUnlocked>, ContractId) {
    let id = Contract::load_from(
        // Relative to hello-sway/call
        "./receiver/out/debug/receiver.bin",
        LoadConfiguration::default(),
    )
    .unwrap()
    .deploy(&wallet, TxParameters::default())
    .await
    .unwrap();

    let instance = Receiver::new(id.clone(), wallet);

    (instance, id.into())
}

async fn get_caller_instance(wallet: WalletUnlocked) -> (Caller<WalletUnlocked>, ContractId) {
    let id = Contract::load_from(
        // Relative to hello-sway/call
        "./caller/out/debug/caller.bin",
        LoadConfiguration::default(),
    )
    .unwrap()
    .deploy(&wallet, TxParameters::default())
    .await
    .unwrap();

    let instance = Caller::new(id.clone(), wallet);

    (instance, id.into())
}

#[tokio::test]
async fn test_call() {
    let wallets = get_wallets().await;
    let (receiver, receiver_id) = get_receiver_instance(wallets[0].clone()).await;
    let (caller, caller_id) = get_caller_instance(wallets[0].clone()).await;

    let call_params = CallParameters::default().set_amount(10);

    let res = caller
        .with_account(wallets[0].clone())
        .unwrap()
        .methods()
        .test_call(receiver_id)
        .call_params(call_params)
        .unwrap()
        .set_contracts(&[&receiver])
        .call()
        .await
        .unwrap();

    println!("{:#?}", res);

    // Check logs
    let logs = res.decode_logs_with_type::<LogReceiverEvent>().unwrap();
    let event = logs.get(0).unwrap();

    println!("{:#?}", event);

    let logs = res.decode_logs_with_type::<LogCallerEvent>().unwrap();
    let event = logs.get(0).unwrap();

    println!("{:#?}", event);
}

#[tokio::test]
async fn test_low_level_call() {
    let wallets = get_wallets().await;
    let (receiver, receiver_id) = get_receiver_instance(wallets[0].clone()).await;
    let (caller, caller_id) = get_caller_instance(wallets[0].clone()).await;

    let selector = fn_selector!(test_func(u64, u64));
    let data = calldata!(1u64, 2u64);

    let call_params = CallParameters::default().set_amount(11);

    let res = caller
        .with_account(wallets[0].clone())
        .unwrap()
        .methods()
        .test_low_level_call(receiver_id, Bytes(selector), Bytes(data))
        .call_params(call_params)
        .unwrap()
        .set_contracts(&[&receiver])
        .call()
        .await
        .unwrap();

    println!("{:#?}", res);

    // Check logs
    let logs = res.decode_logs_with_type::<LogReceiverEvent>().unwrap();
    let event = logs.get(0).unwrap();

    println!("{:#?}", event);
}

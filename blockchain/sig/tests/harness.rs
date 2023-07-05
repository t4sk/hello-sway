use fuels::{
    accounts::{fuel_crypto, fuel_crypto::fuel_types::Bytes64},
    prelude::*,
    types::{Bits256, ContractId, B512},
};

// Load abi from json
abigen!(Contract(
    name = "MyContract",
    abi = "out/debug/sig-abi.json"
));

async fn get_contract_instance() -> (MyContract<WalletUnlocked>, ContractId, Vec<WalletUnlocked>) {
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
    let wallet = &wallets[0];

    let id = Contract::load_from("./out/debug/sig.bin", LoadConfiguration::default())
        .unwrap()
        .deploy(wallet, TxParameters::default())
        .await
        .unwrap();

    let instance = MyContract::new(id.clone(), wallet.clone());

    (instance, id.into(), wallets)
}

#[tokio::test]
async fn test_sig() {
    let (instance, id, wallets) = get_contract_instance().await;

    // Get tx hash
    let data: u64 = 123;

    let res = instance.methods().get_hash(data).call().await.unwrap();

    // Test invalid sig
    // let data: u64 = 124;

    // data to big endian bytes
    let data_bytes = data.to_be_bytes();
    let msg_hash = fuel_crypto::Message::new(data_bytes.clone());

    // Sign
    let sig = wallets[0].sign_message(&data_bytes.clone()).await.unwrap();
    let sig_bytes = Bytes64::from(sig);
    let b512_sig = B512::from((
        Bits256(sig_bytes[..32].try_into().unwrap()),
        Bits256(sig_bytes[32..].try_into().unwrap()),
    ));

    let res = instance
        .methods()
        .test_sig(b512_sig.clone(), res.value)
        .call()
        .await
        .unwrap();

    println!("signer {:?}", wallets[0].address().hash());
    println!("recovered {:?}", res.value);

    assert_eq!(res.value, Address::from(wallets[0].address()));
}

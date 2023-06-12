use fuels::{
    core::{calldata},
    accounts::fuel_crypto,
    accounts::{fuel_crypto::{fuel_types::Bytes64}},
    prelude::*,
    types::ContractId,
    types::{Bits256, B512},
};

use sha2::{Digest, Sha256};

// Load abi from json
abigen!(Contract(
    name = "MyContract",
    abi = "out/debug/sig-abi.json"
));

async fn get_contract_instance() -> (MyContract<WalletUnlocked>, ContractId, WalletUnlocked) {
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

    let id = Contract::load_from("./out/debug/sig.bin", LoadConfiguration::default())
        .unwrap()
        .deploy(&wallet, TxParameters::default())
        .await
        .unwrap();

    let instance = MyContract::new(id.clone(), wallet.clone());

    (instance, id.into(), wallet)
}

#[tokio::test]
async fn test_sig() {
    let (inst, inst_id, wallet) = get_contract_instance().await;

    let x: u64 = 2;
    let y: u64 = 3;
    let data = Bytes(vec![1,2,3]);
    let params = SignParams {
        contract_id: inst_id,
        z: 123,
    };

    // Hash data
    let mut hasher = Sha256::new();
    hasher.update(data.0.clone());
    let data_hash: [u8; 32] = hasher.finalize().try_into().unwrap();

    let res = inst.methods().get_data_hash(data.clone()).call().await.unwrap();
    println!("data:  {:?}", data);
    println!("data hash: {:?}", data_hash);
    println!("res:       {:?}", res.value);

    let data_bytes = calldata!((x, y, params.clone(), Bits256(data_hash.clone())));
    // println!("calldata {:?}", data_bytes.clone());

    // Sign
    let sig = wallet.sign_message(&data_bytes).await.unwrap();
    let sig_bytes = Bytes64::from(sig);
    let sig_b512 = B512::from((
        Bits256(sig_bytes[..32].try_into().unwrap()),
        Bits256(sig_bytes[32..].try_into().unwrap()),
    ));

    // Recover
    // NOTE - data_bytes is hashed by Message::new
    let msg_hash = fuel_crypto::Message::new(data_bytes);
    let rec = sig.recover(&msg_hash).unwrap();

    println!("wallet:    0x{:?}", wallet.clone().address().hash());
    println!("recovered: 0x{:?}", rec.hash());

    // Compare msg hashes
    let res = inst.methods().get_msg_hash(x, y, params.clone(), data.clone()).call().await.unwrap();
    println!("msg_hash:  {:?}", msg_hash);
    println!("res.value: {:?}", fuel_crypto::Message::from_bytes(res.value.0));

    // Recover signer from sway contract
    let res = inst.methods().recover(sig_b512, x, y, params.clone(), data.clone()).call().await.unwrap();
    println!("recovered: {:#?}", res.value);
}

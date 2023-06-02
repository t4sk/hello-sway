use fuels::{prelude::*, types::ContractId, types::Identity};

// Load abi from json
abigen!(Contract(
    name = "NFT",
    abi = "out/debug/nft-abi.json"
));

async fn set_up() -> (NFT<WalletUnlocked>, ContractId, Vec<WalletUnlocked>) {
    // Launch a local network and deploy the contract
    let mut wallets = launch_custom_provider_and_get_wallets(
        WalletsConfig::new(
            Some(5),             /* Single wallet */
            Some(1),             /* Single coin (UTXO) */
            Some(1_000_000_000), /* Amount per coin */
        ),
        None,
        None,
    )
    .await;
    let wallet = &wallets[0];

    let id = Contract::load_from(
        "./out/debug/nft.bin",
        LoadConfiguration::default(),
    )
    .unwrap()
    .deploy(wallet, TxParameters::default())
    .await
    .unwrap();

    let instance = NFT::new(id.clone(), wallet.clone());

    (instance, id.into(), wallets)
}

#[tokio::test]
async fn test_mint() {
    let (instance, contract_id, wallets) = set_up().await;

    let to = Identity::Address(wallets[0].address().into());

    // Mint
    let res = instance
        .methods()
        .mint(to.clone())
        .call()
        .await
        .unwrap();

    let token_id = res.value;
    assert_eq!(token_id, 1);

    // Check owner
    let res = instance
        .methods()
        .owner_of(token_id)
        .call()
        .await
        .unwrap();

    assert_eq!(res.value.unwrap(), to);

    // Check balance
    let res = instance
        .methods()
        .balance_of(to)
        .call()
        .await
        .unwrap();

    assert_eq!(res.value, 1);
}

#[tokio::test]
async fn test_transfer_from() {
    let (instance, contract_id, wallets) = set_up().await;

    let owner = &wallets[0];
    let operator = &wallets[1];
    let recipient = &wallets[2];
    let owner_id = Identity::Address(owner.address().into());
    let operator_id = Identity::Address(operator.address().into());
    let recipient_id = Identity::Address(recipient.address().into());

    // Mint
    let res = instance
        .with_account(owner.clone())
        .unwrap()
        .methods()
        .mint(owner_id.clone())
        .call()
        .await
        .unwrap();

    let token_id = res.value;

    instance
        .with_account(owner.clone())
        .unwrap()
        .methods()
        .set_approval_for_all(operator_id.clone(), true)
        .call()
        .await
        .unwrap();

    let res = instance
        .with_account(operator.clone())
        .unwrap()
        .methods()
        .transfer_from(owner_id.clone(), recipient_id.clone(), token_id)
        .call()
        .await
        .unwrap();

    // Check log
    let logs = res.decode_logs_with_type::<TransferEvent>().unwrap();
    let event = logs.get(0).unwrap();

    assert_eq!(
        *event,
        TransferEvent {
            token_id,
            from: Some(owner_id.clone()),
            to: Some(recipient_id.clone())
        }
    );

    // Check owner
    let res = instance
        .methods()
        .owner_of(token_id)
        .call()
        .await
        .unwrap();

    assert_eq!(res.value.unwrap(), recipient_id.clone());
}
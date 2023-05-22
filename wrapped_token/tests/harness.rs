
use fuels::{prelude::*, tx::ContractId, types::AssetId};

// Load abi from json
abigen!(Contract(
    name = "WrappedToken",
    abi = "out/debug/wrapped_token-abi.json"
));

async fn get_contract_instance() -> (WrappedToken<WalletUnlocked>, ContractId, Vec<WalletUnlocked>) {
    // Launch a local network and deploy the contract
    let mut wallets = launch_custom_provider_and_get_wallets(
        WalletsConfig::new(
            Some(2),             /* Single wallet */
            Some(1),             /* Single coin (UTXO) */
            Some(1_000_000_000), /* Amount per coin */
        ),
        None,
        None,
    )
    .await;
    let wallet = &wallets[0];

    let id = Contract::deploy(
        "./out/debug/wrapped_token.bin",
        wallet,
        DeployConfiguration::default(),
    )
    .await
    .unwrap();

    let instance = WrappedToken::new(id.clone(), wallet.clone());

    (instance, id.into(), wallets)
}

// TODO: test reverts

#[tokio::test]
async fn test_deposit_and_withdraw_to_address() {
    let (instance, contract_id, wallets) = get_contract_instance().await;
    let asset_id = AssetId::new(contract_id.into());

    // deposit
    let call_params = CallParameters::default().set_amount(10);

    instance
        .methods()
        .deposit()
        .call_params(call_params)
        .unwrap()
        .append_variable_outputs(1)
        .call()
        .await
        .unwrap();

    let bal = wallets[0].get_asset_balance(&asset_id).await.unwrap();
    assert_eq!(bal, 10);

    // withdraw
    let call_params = CallParameters::default()
        .set_amount(1)
        .set_asset_id(asset_id);

    instance
        .methods()
        .withdraw()
        .call_params(call_params)
        .unwrap()
        .append_variable_outputs(1)
        .call()
        .await
        .unwrap();

    let bal = wallets[0].get_asset_balance(&asset_id).await.unwrap();
    assert_eq!(bal, 9);
}

// #[tokio::test]
// async fn test_mint_and_transfer_to_contract() {
//     let (instance, contract_id, wallets) = get_contract_instance().await;
//     let asset_id = AssetId::new(contract_id.into());

//     // mint
//     instance.methods().mint(100).call().await.unwrap();

//     // burn
//     instance.methods().burn(10).call().await.unwrap();

//     // mint to address
//     instance
//         .methods()
//         .mint_to_address(10, Address::from(wallets[0].address()))
//         .append_variable_outputs(1)
//         .call()
//         .await
//         .unwrap();

//     // mint to contract
//     instance
//         .methods()
//         .mint_to_contract(100, contract_id)
//         .call()
//         .await
//         .unwrap();

//     // transfer to address
//     instance
//         .methods()
//         .transfer_to_address(10, contract_id, Address::from(wallets[0].address()))
//         .append_variable_outputs(1)
//         .call()
//         .await
//         .unwrap();

//     let bal = wallets[0].get_asset_balance(&asset_id).await.unwrap();
//     println!("WALLET BALANCE {:?}", bal);

//     // force transfer to contract
//     instance
//         .methods()
//         .force_transfer_to_contract(10, contract_id, contract_id)
//         .call()
//         .await
//         .unwrap();

//     let res = instance
//         .methods()
//         .get_balance_of_contract(contract_id, contract_id)
//         .call()
//         .await
//         .unwrap();

//     println!("CONTRACT BALANCE {:?}", res.value);
// }

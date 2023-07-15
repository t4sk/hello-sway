use fuels::accounts::predicate::Predicate;
use fuels::{prelude::*, types::ContractId};

// Load abi from json
abigen!(Predicate(name = "Otc", abi = "out/debug/otc-abi.json"));

// The fee-paying base asset
const BASE_ASSET: AssetId = AssetId::new([0u8; 32]);
const SELL_ASSET: AssetId = AssetId::new([1u8; 32]);
const ASK_ASSET: AssetId = AssetId::new([2u8; 32]);
const SELL_AMOUNT: u64 = 100;
const ASK_AMOUNT: u64 = 200;

async fn get_wallets() -> Vec<WalletUnlocked> {
    let assets = [BASE_ASSET, SELL_ASSET, ASK_ASSET]
        .map(|asset| AssetConfig {
            id: asset,
            num_coins: 1,
            coin_amount: 1_000_000,
        })
        .to_vec();

    let wallets = launch_custom_provider_and_get_wallets(
        WalletsConfig::new_multiple_assets(5, assets),
        None,
        None,
    )
    .await;

    wallets
}

// Get the balance of a given token of an address
async fn get_balance(provider: &Provider, address: &Bech32Address, asset: AssetId) -> u64 {
    provider.get_asset_balance(address, asset).await.unwrap()
}

#[tokio::test]
async fn test_predicate() {
    let wallets = get_wallets().await;

    // let bal = wallets[0].get_balances().await.unwrap();
    // dbg!(bal);

    let provider = wallets[0].try_provider().unwrap().clone();

    let config = OtcConfigurables::new()
        .set_ASK_AMOUNT(ASK_AMOUNT)
        .set_ASK_TOKEN(ContractId::from(*ASK_ASSET))
        .set_RECEIVER(wallets[0].address().into());

    let predicate = Predicate::load_from("out/debug/otc.bin")
        .unwrap()
        .with_provider(provider.clone())
        .with_configurables(config);

    // Check predicate balance.
    wallets[0]
        .transfer(predicate.address(), SELL_AMOUNT, SELL_ASSET, TxParameters::default())
        .await
        .unwrap();

    let bal = predicate.get_asset_balance(&SELL_ASSET).await.unwrap();
    println!("otc balance: {:?}", bal);

    // Get predicate coin to unlock
    let predicate_coins = &provider
        .get_spendable_resources(ResourceFilter {
            from: predicate.address().clone(),
            asset_id: SELL_ASSET,
            amount: SELL_AMOUNT,
            excluded_utxos: vec![],
            excluded_message_ids: vec![],
        })
        .await
        .unwrap();

    dbg!(predicate_coins);
    // let predicate_coin_utxo_id = match predicate_coin {
    //     Resource::Coin(coin) => coin.utxo_id,
    //     _ => panic!(),
    // };

    // // Offered asset coin belonging to the predicate root
    // let input_predicate = Input::CoinPredicate {
    //     utxo_id: predicate_coin_utxo_id,
    //     tx_pointer: TxPointer::default(),
    //     owner: predicate.address().into(),
    //     amount: offered_amount,
    //     asset_id: SELL_ASSET,
    //     maturity: 0,
    //     predicate: predicate.code(),
    //     predicate_data: vec![],
    // };

    // // Use a change output to send the unlocked coins back to the wallet
    // let output_offered_change = Output::Change {
    //     to: Address::from(wallet.address()),
    //     amount: 0,
    //     asset_id: SELL_ASSET,
    // };

    // let script_call = ScriptCallHandler::<()>::new(
    //     vec![],
    //     UnresolvedBytes::default(),
    //     wallet.clone(),
    //     provider.clone(),
    //     Default::default(),
    // )
    // .with_inputs(vec![input_predicate])
    // .with_outputs(vec![output_offered_change])
    // .tx_params(TxParameters::new(Some(1), Some(10_000_000), None));

    // let _response = script_call.call().await.unwrap();

    // // The predicate root's coin has been spent
    // let predicate_balance = get_balance(provider, predicate.address(), SELL_ASSET).await;
    // assert_eq!(predicate_balance, 0);

    // // Wallet balance is the same as before it sent the coins to the predicate
    // let wallet_balance = get_balance(provider, wallet.address(), SELL_ASSET).await;
    // assert_eq!(wallet_balance, initial_wallet_balance);
}

use defi_kit::{helpers, tx_builder};
use log::info;
use std::error::Error;
use web3::transports::http::Http;
use web3::Web3;
use dialoguer::Confirm;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv::dotenv().ok();
    env_logger::init();

    let wallet_address = get_var("WALLET_ADDRESS");
    let chain = get_var("CHAIN");
    let input_token = get_var("INPUT_TOKEN");
    let output_token = get_var("OUTPUT_TOKEN");
    let amount = get_var("AMOUNT");
    let slippage = get_var("SLIPPAGE");

    let node_url = get_var("NODE_URL");
    let w = {
        let t = Http::new(&node_url).expect("could not connect to node");
        Web3::new(t)
    };
    info!("connected to node");

    let chain_id = w.eth().chain_id().await?.as_u128();
    info!("chain id: {}", chain_id);

    let b = tx_builder::TxBuilder::new();
    let tx = b
        .transact(
            &wallet_address,
            &chain,
            &input_token,
            &output_token,
            &amount,
            &slippage,
        )
        .await
        .expect("could not build tx");
    info!("tx received from builder");

    let nonce = w
        .eth()
        .transaction_count(helpers::parse_address(&wallet_address), None)
        .await
        .expect("could not get wallet nonce")
        .as_u128();
    info!("nonce: {}", nonce);

    let to = helpers::parse_address_20(&tx.to);
    info!("to: {}", &tx.to);

    let value = u128::from_str_radix(&tx.value, 10).expect("could not parse tx value");
    info!("value: {}", value);

    let gas_price = w
        .eth()
        .gas_price()
        .await
        .expect("could not retrieve gas price")
        .as_u128();
    info!("gas price: {}", gas_price);

    let gas_limit = tx.gas;
    info!("gas limit: {}", gas_limit);

    let data = {
        let s = tx.data.trim_start_matches("0x");
        hex::decode(s).expect("could not parse data")
    };
    info!("data is parsed");

    let raw_tx =
        ethereum_tx_sign::RawTransaction::new(nonce, to, value, gas_price, gas_limit, data);
    info!("raw tx is built");

    let private_key = {
        let s = get_var("PRIVATE_KEY");
        hex::decode(s).expect("could not decode private key")
    };
    info!("parsed private key");

    let signed_tx = raw_tx.sign(&private_key, &chain_id);
    info!("tx is signed");

    if Confirm::new().with_prompt("Submit tx?").interact()? {
        let res = w.eth().send_raw_transaction(signed_tx.into()).await.expect("failed to submit tx");
        info!("tx is submitted: {}", res.to_string());
    } else {
        info!("not submitting anything");
    }

    Ok(())
}

fn get_var(name: &str) -> String {
    std::env::var(name).expect(&format!("`{}` is not set", name))
}

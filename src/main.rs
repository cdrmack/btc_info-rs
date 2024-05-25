#[macro_use]
extern crate serde_derive;

use dotenv;
use reqwest;
use tokio;
use serde_json;

mod blockchain_status;

use crate::blockchain_status::BlockchainStatus;

const ENDPOINT: &str = "https://btcbook.nownodes.io/api/";

#[tokio::main]
async fn send_request(url: &str) -> String {
    let api_key = dotenv::var("API_KEY").expect("Cannot find API_KEY");
    let client = reqwest::Client::new();

    client
	.get(url)
	.header("api-key", api_key)
	.send()
	.await
	.expect("Failed to get response")
	.text()
	.await
	.expect("Failed to convert payload")
}

fn main() {
    let response = send_request(&ENDPOINT);
    let parsed: BlockchainStatus = serde_json::from_str(&response).expect("Cannot parse response (JSON)");
    println!("QUERY {} (chain: {})", parsed.blockbook.coin, parsed.backend.chain);
}

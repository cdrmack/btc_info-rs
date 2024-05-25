#[macro_use]
extern crate serde_derive;

use dotenv;
use reqwest;
use tokio;
use serde_json;

mod blockchain_status;
mod blockchain_address;

use crate::blockchain_status::BlockchainStatus;
use crate::blockchain_address::BlockchainAddress;

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

fn blockchain_status_request() -> BlockchainStatus {
    let response = send_request(&ENDPOINT);
    serde_json::from_str(&response).expect("Cannot parse response (JSON)")
}

fn blockchain_address_request(address: &str) -> BlockchainAddress {
    let response = send_request(&[ENDPOINT, "v2/address/", &address].join(""));
    serde_json::from_str(&response).expect("Cannot parse response (JSON)")
}

fn main() {
    let blockchain_status = blockchain_status_request();
    println!("QUERY {} (chain: {})", blockchain_status.blockbook.coin, blockchain_status.backend.chain);

    let address = dotenv::var("ADDRESS").expect("Cannot find ADDRESS");
    let address_status = blockchain_address_request(&address);
    println!("QUERY address {}", address_status.address);
    for (pos, txid) in address_status.txids.iter().enumerate() {
	println!("{}: {}", pos, txid);
    }
}

use dotenv;
use reqwest;
use tokio;

const ENDPOINT: &str = "https://btcbook.nownodes.io/api/";

#[tokio::main]
async fn main() {
    let api_key = dotenv::var("API_KEY").expect("Cannot find API_KEY");
    let client = reqwest::Client::new();

    let response = client
	.get(ENDPOINT)
	.header("api-key", api_key)
	.send()
	.await
	.expect("Failed to get response")
	.text()
	.await
	.expect("Failed to convert payload");

    println!("response = {:?}", response);
}

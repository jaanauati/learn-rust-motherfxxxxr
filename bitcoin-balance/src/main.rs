use reqwest;
use serde_json::Value;

const ENDPOINT : &str = "https://btcscan.org/api/address";

// random coinbase address.
const ADDRESS: &str = "bc1qwzrryqr3ja8w7hnja2spmkgfdcgvqwp5swz4af4ngsjecfz0w0pqud7k38";

// reqwest is non-blocking by default so we need to declare an async function.
async fn get_address_balance(address: &str) -> Result<i64, reqwest::Error> {
    let url = format!("{}/{}", ENDPOINT, address);
    let response = reqwest::get(url).await?;
    let json = response.json::<Value>().await?;
    // {"address": String("bc1qwzrryqr3ja8w7hnja2spmkgfdcgvqwp5swz4af4ngsjecfz0w0pqud7k38"), "chain_stats": Object {"funded_txo_count": Number(9493), "funded_txo_sum": Number(3027578368819), "spent_txo_count": Number(9376), "spent_txo_sum": Number(2990640339386), "tx_count": Number(9695)}, "mempool_stats": Object {"funded_txo_count": Number(0), "funded_txo_sum": Number(0), "spent_txo_count": Number(0), "spent_txo_sum": Number(0), "tx_count": Number(0)}}
    // This function returns Result<...> so we can't do question mark as_i64()?
    // for the following two since they return Option<i64>.
    let funded = match json["chain_stats"]["funded_txo_sum"].as_i64() {
        Some(value) => value,
        _ => 0
    };
    let spent = match json["chain_stats"]["spent_txo_sum"].as_i64() {
        Some(value) => value,
        _ => 0,
    };
    Ok(funded - spent)
}

// tokio is an async runtime for rust.
#[tokio::main]
async fn main() {
    match get_address_balance(ADDRESS).await {
        Ok(balance) => println!("Balance: {:?}", balance),
        Err(e) => println!("Error: {}", e),
    }
}

use std::collections::HashMap;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Response {
    product_code: String,
    total_bid_depth: f64
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get("https://api.bitflyer.jp/v1/ticker/")
        .await?
        .json::<Response>()
        .await?;
    println!("{:#?}", resp);
    println!("{:#?}", resp.total_bid_depth);
    Ok(())
}

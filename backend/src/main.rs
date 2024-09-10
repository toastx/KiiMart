mod contracts;
mod database;
use alloy::primitives::address;
use anyhow;
use dotenv::dotenv;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let cid = database::ipfs::upload_data(
        env::var("pinata_jwt").map_err(|_| anyhow::anyhow!("pinata_jwt is not present"))?,
        "D:/hackahton/rwamarketplace/backend/src/image.png".to_string(),
    )
    .await;
    println!("cid: {:?}", cid);
    let file = database::ipfs::fetch_data(
        cid.unwrap(),
        env::var("pinata_jwt").map_err(|_| anyhow::anyhow!("pinata_jwt is not present"))?,
    )
    .await;
    println!("file: {:?}", file);
    Ok(())
}

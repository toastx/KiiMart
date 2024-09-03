mod contracts;
use alloy::primitives::address;





#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let contract = contracts::load_contract().await?;
    let buyer_address = address!("104dc4c1FcA6359B9bdBf81705E34f1ba91a3958");
    let seller = address!("F890F95047D40e59c42a3E6d5720a89EE29453cE");
    let seller_private_key = contracts::generate_private_key("3f7e255c7a960413344eb493980a17696d97f94285b443891184aa15d767a04d");
    let wallet = contracts::generate_wallet_obj(seller_private_key);
    let buyer_amount = 220;
    let seller_amount = 200;
    let order_id = "test".to_string();
    
    contracts::create_escrow(&contract, buyer_address, seller, buyer_amount, seller_amount, order_id,wallet).await?;
    Ok(())
}
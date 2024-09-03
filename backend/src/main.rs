mod contracts;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    contracts::load_contract().await?;
    Ok(())
}
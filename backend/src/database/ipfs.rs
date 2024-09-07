use pinata_sdk::PinataApi;
use anyhow::Error;

pub async fn init_client() -> Result<PinataApi, anyhow::Error>{
    let pinata_client = PinataApi::new("api_key", "secret_api_key").unwrap();
    
    let result = pinata_client.test_authentication().await;
    if let Ok(_) = result {
        return Ok(pinata_client);
    }
    else {
        return Err(Error::new(result.unwrap_err()));
    }
}



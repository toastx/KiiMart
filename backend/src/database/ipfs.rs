use pinata_sdk::{ApiError, PinataApi};

pub async fn init_client(api_key:String,secret_api_key:String) -> Result<PinataApi, ApiError>{
    let pinata_client = PinataApi::new(api_key, secret_api_key).unwrap();
    
    let result = pinata_client.test_authentication().await;
    println!("result: {:?}", result);
    if let Ok(_) = result {
        return Ok(pinata_client);
    }
    else{
        return Err(result.unwrap_err());
    }
    
}

pub async fn upload_file(client:PinataApi, file_path:String) -> Result<String, ApiError>{
    todo!("not implemented")
}



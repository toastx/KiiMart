use pinata_sdk::{ApiError, PinByFile, PinByJson, PinataApi};
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};
use reqwest::Client;
use std::collections::HashMap;

pub async fn init_client(api_key: String, secret_api_key: String) -> Result<PinataApi, ApiError> {
    let pinata_client = PinataApi::new(api_key, secret_api_key).unwrap();

    let result = pinata_client.test_authentication().await;
    println!("result: {:?}", result);
    if let Ok(_) = result {
        return Ok(pinata_client);
    } else {
        return Err(result.unwrap_err());
    }
}

pub async fn upload_data(client: PinataApi, file_path: String) -> Result<String, ApiError> {
    let result = client.pin_file(PinByFile::new(file_path)).await;
    if let Ok(pinned_object) = result {
        let hash = pinned_object.ipfs_hash;
        return Ok(hash);
    } else {
        return Err(result.unwrap_err());
    }
}

pub async fn fetch_data(client: PinataApi, cid: String, apitoken: String) -> Result<String, ApiError> {
    let id = cid;
    let token = apitoken;
    let url = format!("https://api.pinata.cloud/v3/files/{}", id);

    let mut headers = HeaderMap::new();
    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_str(&format!("Bearer {}", token)).unwrap(),
    );

    let client = Client::new();
    let response = client.get(&url).headers(headers).send().await.unwrap();

    let body = response.text().await.unwrap();
    println!("{}", body);

    Ok(body)
}

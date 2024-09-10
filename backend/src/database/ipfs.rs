use pinata_sdk::ApiError;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};
use reqwest::multipart;
use reqwest::Client;

pub async fn upload_data(apitoken: String, file_path: String) -> Result<String, ApiError> {
    let token = apitoken;
    let url = "https://api.pinata.cloud/v3/files";
    let form = multipart::Form::new().text("file", file_path);

    let mut headers = HeaderMap::new();
    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_str(&format!("Bearer {}", token)).unwrap(),
    );

    let client = Client::new();
    let response = client
        .post(url)
        .headers(headers)
        .multipart(form)
        .send()
        .await
        .unwrap();

    let response = response.json::<serde_json::Value>().await.unwrap();
    let body = response.to_string();
    println!("{}", body);
    println!("ID: {}", response["id"].to_string());

    Ok(response["id"].to_string())
}

pub async fn fetch_data(cid: String, apitoken: String) -> Result<String, ApiError> {
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

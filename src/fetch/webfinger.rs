use std::error::Error;

use reqwest::StatusCode;
use serde::de::DeserializeOwned;

pub async fn fetch_webfinger_resource<T: DeserializeOwned>(
    client: &reqwest::Client,
    uri: &str,
) -> Result<T, Box<dyn Error>> {
    let current_uri: &str = uri;
    loop {
        let response = client.get(current_uri).send().await?;
        match response.status() {
            StatusCode::OK => {
                // continue
            }
            x @ (StatusCode::FOUND | StatusCode::TEMPORARY_REDIRECT) => {
                // TODO: support redirection
                return Err(format!("Not supported redirection: status={x}").into());
            }
            x => return Err(format!("Unknown response: status={x}").into()),
        }

        let data: T = response.json().await?;

        return Ok(data);
    }
}

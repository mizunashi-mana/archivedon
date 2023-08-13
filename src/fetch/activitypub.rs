use archivedon::activitypub::actor::Actor;
use reqwest::StatusCode;
use serde::de::DeserializeOwned;
use std::error::Error;

pub async fn fetch_actor(client: reqwest::Client, uri: String) -> Result<Actor, Box<dyn Error>> {
    fetch_ap_resource(client, uri).await
}

pub async fn fetch_ap_resource<T: DeserializeOwned>(
    client: reqwest::Client,
    uri: String,
) -> Result<T, Box<dyn Error>> {
    let current_uri: String = uri;
    loop {
        let response = client
            .get(current_uri)
            .header(reqwest::header::ACCEPT, "application/activity+json")
            .send()
            .await?;
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

use archivedon::activitypub::json as ap_json;
use archivedon::activitypub::model as ap_model;
use reqwest::StatusCode;
use serde::de::DeserializeOwned;
use std::error::Error;

pub async fn fetch_actor(
    client: &reqwest::Client,
    uri: String,
) -> Result<ap_model::Object, Box<dyn Error>> {
    let object: ap_model::Object =
        ap_json::ModelConv::to_model(fetch_ap_resource(client, uri).await?)?;
    if object.actor_items.is_none() {
        Err(format!("Actor items are should be available.").into())
    } else {
        Ok(object)
    }
}

pub async fn fetch_object(
    client: &reqwest::Client,
    uri: String,
) -> Result<ap_model::Object, Box<dyn Error>> {
    let object: ap_model::Object =
        ap_json::ModelConv::to_model(fetch_ap_resource(client, uri).await?)?;
    Ok(object)
}

pub async fn fetch_ap_resource<T: DeserializeOwned>(
    client: &reqwest::Client,
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

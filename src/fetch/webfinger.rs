use std::error::Error;

use reqwest::StatusCode;
use serde::de::DeserializeOwned;

use archivedon::webfinger::resource::Resource as WebfingerResource;
use url::Url;

use crate::fetch::webfinger;

pub async fn fetch_ap_account_actor_url(
    client: &reqwest::Client,
    domain: &str,
    subject: &str,
) -> Result<String, Box<dyn Error>> {
    let webfinger_url = Url::parse_with_params(
        &format!("https://{domain}/.well-known/webfinger"),
        &[("resource", subject)],
    )?;
    let acct: WebfingerResource =
        webfinger::fetch_webfinger_resource(client, webfinger_url.as_str()).await?;

    let self_link = match acct
        .links
        .and_then(|links| links.into_iter().find(|link| link.rel == "self"))
    {
        None => return Err(format!("The self link is not found: {subject}").into()),
        Some(x) => x,
    };

    let is_valid_type = match &self_link.typ {
        None => true,
        Some(link_type) => link_type == "application/activity+json",
    };
    if !is_valid_type {
        return Err(format!(
            "The self link is not an actor of ActivityPub: {subject}, type={:?}",
            &self_link.typ
        )
        .into());
    }

    let acct_actor_url = match self_link.href {
        None => return Err(format!("The self link does not have a URL: {subject}").into()),
        Some(x) => x,
    };

    Ok(acct_actor_url)
}

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

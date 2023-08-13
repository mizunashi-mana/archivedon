use std::error::Error;
use archivedon::activitypub::actor::Actor;

use crate::fetch;

pub async fn fetch_actor(client: reqwest::Client, uri: String) -> Result<Actor, Box<dyn Error>> {
    fetch::fetch_ap_resource(client, uri).await
}

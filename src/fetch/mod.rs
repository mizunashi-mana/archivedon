use std::error::Error;
use url::Url;

mod activitypub;
mod input;
mod webfinger;

use archivedon::webfinger::resource::Resource as WebfingerResource;

pub async fn run(input_path: &str) -> Result<(), Box<dyn Error>> {
    let input = input::load(input_path).await?;
    let client = reqwest::Client::new();

    for account in input.accounts {
        fetch_account(&client, &account).await?;
    }

    Ok(())
}

pub async fn fetch_account(client: &reqwest::Client, account: &str) -> Result<(), Box<dyn Error>> {
    let account_stripped = account.strip_prefix("@").unwrap_or(&account);
    let (username, domain) = match account_stripped.split_once("@") {
        None => return Err(format!("Illegal account: {account}").into()),
        Some(x) => x,
    };

    let webfinger_url = Url::parse_with_params(
        &format!("https://{domain}/.well-known/webfinger"),
        &[("resource", format!("acct:{username}@{domain}"))],
    )?;
    let acct: WebfingerResource =
        webfinger::fetch_webfinger_resource(client, webfinger_url.as_str()).await?;
    println!("{:?}", &acct);

    Ok(())
}

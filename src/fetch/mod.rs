use std::error::Error;

mod activitypub;
mod input;
mod webfinger;

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

    let account_actor_url = webfinger::fetch_ap_account_actor_url(client, username, domain).await?;
    let account_actor = activitypub::fetch_actor(client, account_actor_url).await?;

    println!("{:?}", account_actor);

    Ok(())
}

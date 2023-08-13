use std::error::Error;

mod fetch;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    fetch::run().await?;

    Ok(())
}

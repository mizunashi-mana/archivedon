use std::error::Error;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Input {
    pub static_base_url: String,
    pub accounts: Vec<String>,
}

pub async fn load(path: &str) -> Result<Input, Box<dyn Error>> {
    let bytes = tokio::fs::read(path).await?;
    let data: Input = serde_json::from_slice(&bytes)?;
    Ok(data)
}

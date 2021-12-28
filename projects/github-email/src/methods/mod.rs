use reqwest::{header::USER_AGENT, Client};
use serde::{Deserialize, Serialize};

use crate::Result;

/// https://api.github.com/users/oovm/events/public
pub async fn get_events() -> Result<String> {
    let out = Client::new().get("https://api.github.com/users/oovm/events/public").header(USER_AGENT, "Octocat").send().await?;
    Ok(out.text().await?)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EventsResponse {
    pub message: String,
    pub documentation_url: String,
}

#[tokio::test]
async fn test() {
    println!("{:#?}", get_events().await)
}

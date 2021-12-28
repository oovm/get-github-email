use crate::{GithubError, Result};
use reqwest::{header::USER_AGENT, Client};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::str::FromStr;

/// https://api.github.com/users/oovm/events/public
pub async fn from_user_events(user: &str) -> Result<String> {
    let url = format!("https://api.github.com/users/{user}/events/public");
    let out = Client::new().get(url).header(USER_AGENT, "Octocat").send().await?;
    let text = out.text().await?;
    let value = Value::from_str(&text)?;
    let first = match &value {
        Value::Array(v) => match v.get(0).and_then(|v| v.as_object()) {
            Some(s) => s,
            None => return Err(GithubError::RuntimeError(format!("User {user} had no public activity."))),
        },
        Value::Object(_) => {
            todo!("{:#?}", value)
        }
        _ => return Err(GithubError::RuntimeError(format!("Unknown response when call from_user_events: {text}"))),
    };
    println!("{:#?}", first);
    Ok(String::new())
}

/// https://api.github.com/users/oovm/events/public
pub async fn get_events2() -> Result<()> {
    let pr = octocrab::instance().events().send().await?;
    println!("{:#?}", pr);
    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EventsResponse {
    pub message: String,
    pub documentation_url: String,
}

#[tokio::test]
async fn test() {
    from_user_events("oovm").await.unwrap();
}

use super::*;

/// Collect authors email from comment
///
/// API: <https://api.github.com/users/{USER}/events/public>
pub async fn collect_user_events(user: &str, authors: &mut Authors) -> Result<()> {
    let url = format!("https://api.github.com/users/{user}/events/public");
    let out = Client::new().get(url).header(USER_AGENT, "octocat").send().await?;
    let text = out.text().await?;
    let value = Value::from_str(&text)?;
    match &value {
        Value::Array(events) => {
            for event in events {
                read_payload(event, authors);
            }
            return Ok(());
        }
        Value::Object(o) => match o.get("message").and_then(|v| v.as_str()) {
            Some(s) => return Err(GithubError::RuntimeError(s.to_string())),
            None => {}
        },
        _ => {}
    };
    Err(GithubError::RuntimeError(format!("Unknown response when call `collect_user_events`: {text}")))
}

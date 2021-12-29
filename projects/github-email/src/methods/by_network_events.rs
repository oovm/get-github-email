use super::*;

/// Collect authors email from comment
///
/// API: <https://api.github.com/networks/{OWNER}/{REPO}/events>
pub async fn collect_network_events(owner: &str, repo: &str, authors: &mut Authors) -> Result<()> {
    let url = format!("https://api.github.com/networks/{owner}/{repo}/events");
    let out = Client::new().get(url).header(USER_AGENT, "octocat").send().await?;
    let text = out.text().await?;
    let value = Value::from_str(&text)?;
    println!("{value:#?}");
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
    Err(GithubError::RuntimeError(format!("Unknown response when call from_user_events: {text}")))
}

fn read_payload(event: &Value, count: &mut Authors) -> Option<()> {
    let payload = event.as_object()?.get("payload")?.as_object()?;
    let commits = payload.get("commits")?.as_array()?;
    for commit in commits {
        match read_commit(commit) {
            Some(s) => count.insert(s),
            None => continue,
        }
    }
    Some(())
}

use std::str::FromStr;

use reqwest::{header::USER_AGENT, Client};
use serde_json::Value;

use crate::{Authors, CommitAuthor, GithubError, Result};

pub mod by_network_events;
pub mod by_repo_events;
pub mod by_user_events;
pub mod for_url_dispatch;

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

fn read_commit(commit: &Value) -> Option<CommitAuthor> {
    let author = commit.as_object()?.get("author")?.as_object()?;
    let name = author.get("name")?.as_str()?.to_string();
    let email = author.get("email")?.as_str()?.to_string();
    Some(CommitAuthor { name, email, count: 1 })
}

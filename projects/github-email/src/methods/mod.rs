use std::{collections::BTreeMap, str::FromStr};

use reqwest::{header::USER_AGENT, Client};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{GithubError, Result};

pub mod by_network_events;
pub mod by_user_events;

fn read_commit(commit: &Value) -> Option<CommitAuthor> {
    let author = commit.as_object()?.get("author")?.as_object()?;
    let name = author.get("name")?.as_str()?.to_string();
    let email = author.get("email")?.as_str()?.to_string();
    Some(CommitAuthor { name, email, count: 1 })
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EventsResponse {
    pub message: String,
    pub documentation_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommitAuthor {
    name: String,
    email: String,
    count: usize,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Authors {
    inner: BTreeMap<String, CommitAuthor>,
}

impl Authors {
    pub fn get(&self, name: &str) -> Option<&CommitAuthor> {
        self.inner.get(name)
    }
    pub fn insert(&mut self, author: CommitAuthor) {
        match self.inner.get_mut(&author.name) {
            Some(s) => s.count += author.count,
            None => self.insert_new(author),
        }
    }
    fn insert_new(&mut self, author: CommitAuthor) {
        self.inner.insert(author.name.clone(), author);
    }
}

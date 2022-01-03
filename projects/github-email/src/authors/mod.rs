use std::{
    collections::{btree_map::Values, BTreeMap},
    fmt::{Debug, Display, Formatter},
};

use serde::{Deserialize, Serialize};
use url::Url;

use crate::{collect_repo_events, collect_user_events, Result};

pub mod query;

#[derive(Default, Serialize, Deserialize)]
pub struct Authors {
    inner: BTreeMap<String, CommitAuthor>,
}

#[derive(Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AuthorQuery {
    Nothing,
    User(String),
    Repo(String, String),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommitAuthor {
    pub name: String,
    pub email: String,
    pub count: usize,
}

impl Authors {
    pub fn get(&self, name: &str) -> Option<&CommitAuthor> {
        self.inner.get(name)
    }
    pub fn insert(&mut self, author: CommitAuthor) {
        match self.inner.get_mut(&author.name) {
            Some(s) => s.count += author.count,
            None => self.insert_force(author),
        }
    }
    pub fn insert_force(&mut self, author: CommitAuthor) {
        self.inner.insert(author.name.clone(), author);
    }
    pub fn items(&self) -> Vec<&CommitAuthor> {
        let mut by_name: Vec<_> = self.into_iter().collect();
        by_name.sort_by_key(|f| -(f.count as isize));
        by_name
    }
}

impl Debug for Authors {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.items().iter()).finish()
    }
}
impl Display for Authors {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.items().iter()).finish()
    }
}

impl<'i> IntoIterator for &'i Authors {
    type Item = &'i CommitAuthor;
    type IntoIter = Values<'i, String, CommitAuthor>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.values()
    }
}

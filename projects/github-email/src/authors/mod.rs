use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Authors {
    inner: BTreeMap<String, CommitAuthor>,
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
            None => self.insert_new(author),
        }
    }
    fn insert_new(&mut self, author: CommitAuthor) {
        self.inner.insert(author.name.clone(), author);
    }
}

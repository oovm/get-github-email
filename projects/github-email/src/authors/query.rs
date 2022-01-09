use log::info;

use crate::{parse_queries, GithubError};

use super::*;

impl Authors {
    pub async fn query<Q>(&mut self, query: Q) -> Result<()>
    where
        Q: Into<AuthorQuery>,
    {
        match query.into() {
            AuthorQuery::Nothing => {}
            AuthorQuery::User(user) => collect_user_events(&user, self).await?,
            AuthorQuery::Repo(user, repo) => collect_repo_events(&user, &repo, self).await?,
        }
        Ok(())
    }
    pub async fn query_many(&mut self, queries: &str) -> Vec<GithubError> {
        let mut errors = vec![];
        for query in parse_queries(queries) {
            info!("Query: {query:?}");
            if let Err(e) = self.query(query).await {
                errors.push(e)
            }
        }
        errors
    }
}

impl AuthorQuery {
    pub fn is_none(&self) -> bool {
        matches!(self, AuthorQuery::Nothing)
    }
    pub fn is_some(&self) -> bool {
        !self.is_none()
    }
}

impl From<&str> for AuthorQuery {
    fn from(value: &str) -> Self {
        let url = match value.contains("://") {
            true => value.to_string(),
            false => format!("https://github.com/{value}"),
        };
        match Url::parse(&url) {
            Ok(o) => AuthorQuery::from(&o),
            Err(_) => AuthorQuery::Nothing,
        }
    }
}

impl From<&Url> for AuthorQuery {
    fn from(value: &Url) -> Self {
        let path = value.path().split("/").collect::<Vec<_>>();
        // println!("{:?} => {:?}", path, path_slice(&path));
        match path_slice(&path) {
            [user] => AuthorQuery::User(user.to_string()),
            [user, repo, ..] => AuthorQuery::Repo(user.to_string(), repo.to_string()),
            _ => AuthorQuery::Nothing,
        }
    }
}

fn path_slice<'v, 's>(path: &'v [&'s str]) -> &'v [&'s str] {
    let mut l = 0;
    let mut r = path.len();
    for ls in path {
        match ls.is_empty() {
            true => l += 1,
            false => break,
        }
    }
    for rs in path.iter().rev() {
        match rs.is_empty() {
            true => r -= 1,
            false => break,
        }
    }
    // println!("{}..{}", l, r);
    &path[l..r]
}

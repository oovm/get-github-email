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
        match Url::parse(value) {
            Ok(o) => AuthorQuery::from(&o),
            Err(_) => AuthorQuery::Nothing,
        }
    }
}

impl From<&Url> for AuthorQuery {
    fn from(value: &Url) -> Self {
        let path = value.path().split("/").collect::<Vec<_>>();
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
    for rs in path {
        match rs.is_empty() {
            true => r -= 1,
            false => break,
        }
    }
    &path[l..r]
}

#[test]
fn test() {
    println!("{:#?}", AuthorQuery::from("https://github.com/oovm/get-github-email"))
}

use super::*;

impl Authors {
    pub async fn query<Q>(&mut self, query: Q) -> Result<()>
    where
        Q: Into<AuthorQuery>,
    {
        match query.as_ref() {
            AuthorQuery::Nothing => {}
            AuthorQuery::User(user) => collect_user_events(&user, self).await?,
            AuthorQuery::Repo(user, repo) => collect_repo_events(&user, &repo, self).await?,
        }
        Ok(())
    }
}

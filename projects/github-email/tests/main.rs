use github_email::AuthorQuery;

#[test]
fn ready() {
    println!("it works!")
}

#[test]
fn user_query() {
    let target = AuthorQuery::User("oovm".to_string());
    assert_eq!(AuthorQuery::from("oovm"), target);
    assert_eq!(AuthorQuery::from("/oovm"), target);
    assert_eq!(AuthorQuery::from("oovm/"), target);
    assert_eq!(AuthorQuery::from("/oovm/"), target);
    assert_eq!(AuthorQuery::from("https://github.com/oovm"), target);
    assert_eq!(AuthorQuery::from("https://github.com/oovm/"), target);
}

#[test]
fn repo_query() {
    let target = AuthorQuery::Repo("oovm".to_string(), "get-github-email".to_string());
    assert_eq!(AuthorQuery::from("oovm/get-github-email"), target);
    assert_eq!(AuthorQuery::from("/oovm/get-github-email"), target);
    assert_eq!(AuthorQuery::from("oovm/get-github-email/"), target);
    assert_eq!(AuthorQuery::from("/oovm/get-github-email/"), target);
    assert_eq!(AuthorQuery::from("https://github.com/oovm/get-github-email"), target);
    assert_eq!(AuthorQuery::from("https://github.com/oovm/get-github-email/"), target);
}

#[tokio::test]
async fn find_email() {
    use github_email::Authors;
    let mut authors = Authors::default();
    let url = r#"
    https://github.com/oovm/
    https://github.com/oovm/get-github-email
    "#;
    for error in authors.query_many(url).await {
        eprintln!("{error}")
    }
    println!("{authors:#?}")
}

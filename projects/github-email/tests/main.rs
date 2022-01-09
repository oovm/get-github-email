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

#[tokio::test]
async fn find_email() {
    use github_email::{parse_queries, Authors};
    let mut authors = Authors::default();
    let url = r#"
    https://github.com/oovm/
    https://github.com/oovm/get-github-email
    "#;
    for query in parse_queries(url) {
        if let Err(e) = authors.query(query).await {
            eprintln!("{e}")
        }
    }
    println!("{authors:#?}")
}

# Find Github Email

Find github user email. 

### API

https://docs.github.com/en/rest/activity/events

### Online



### Rust




```rust
#[tokio::test]
async fn find_email() {
    use github_email::{parse_queries, Authors};
    let mut authors = Authors::default();
    let url = r#"
    https://github.com/oovm/
    https://github.com/oovm/get-github-email
    "#;
    for query in parse_queries(url) {
        authors.query(query).await.ok();
    }
    println!("{authors}")
}
```
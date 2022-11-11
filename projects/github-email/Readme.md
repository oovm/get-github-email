# Find Github Email

Find github user email.

### Online

https://oovm.github.io/get-github-email/

### Rust

```bash
github-email = "*"
```

```rust
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
```

### API Reference

https://docs.github.com/en/rest/activity/events
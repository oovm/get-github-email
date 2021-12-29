use github_email::{collect_network_events, Authors};

#[test]
fn ready() {
    println!("it works!")
}

#[tokio::test]
async fn test() {
    let mut authors = Authors::default();
    collect_network_events("oovm", "get-github-email", &mut authors).await.unwrap();
    println!("{authors:#?}")
}

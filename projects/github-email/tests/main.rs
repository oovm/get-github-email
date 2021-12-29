use github_email::{collect_network_events, collect_user_events, Authors};

#[test]
fn ready() {
    println!("it works!")
}

#[tokio::test]
async fn test() {
    let mut authors = Authors::default();
    collect_user_events("oovm", &mut authors).await.unwrap();
    collect_network_events("oovm", "get-github-email", &mut authors).await.unwrap();
    println!("{authors:#?}")
}

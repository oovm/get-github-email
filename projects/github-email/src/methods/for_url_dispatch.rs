use reqwest::Url;

use super::*;

impl Authors {}

#[test]
fn url() -> Result<()> {
    Url::parse("https://github.com/oovm")?;

    Ok(())
}

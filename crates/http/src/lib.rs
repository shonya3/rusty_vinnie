const USER_AGENT: &str =
    "rusty_vinnie/0.1 https://github.com/shonya3/rusty_vinnie (email: poeshonya3@gmail.com)";

/// Setups client with user agent
pub fn client() -> reqwest::Client {
    reqwest::ClientBuilder::new()
        .user_agent(USER_AGENT)
        .build()
        .unwrap()
}

/// Fetches as text from given url.
pub async fn text(url: &str) -> Result<String, reqwest::Error> {
    client()
        .get(url)
        .send()
        .await?
        .error_for_status()?
        .text()
        .await
}

use reqwest::Client;

const USER_AGENT: &str = "rusty_vinnie/0.1 (contact: poeshonya3@gmail.com)";

pub fn client() -> Client {
    reqwest::ClientBuilder::new()
        .user_agent(USER_AGENT)
        .build()
        .unwrap()
}

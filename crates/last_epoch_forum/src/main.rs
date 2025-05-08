pub mod content;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    // let html = std::fs::read_to_string("1.2.3.html").unwrap();

    // let markdown = content::get_content(&html).unwrap();

    // std::fs::write("1-2-3.md", &markdown).unwrap();

    Ok(())
}

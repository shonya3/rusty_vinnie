use diablo::PostCategory;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Serialize, Deserialize)]
struct Post {
    category_id: PostCategory,
}

#[tokio::main]
async fn main() {
    let json = json!({
        "category_id": 65535
    });

    println!("{:#?}", serde_json::from_value::<Post>(json).unwrap());

    println!(
        "{}",
        serde_json::to_string(&Post {
            category_id: PostCategory::Other
        })
        .unwrap()
    );
}

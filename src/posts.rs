use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Post {
    pub title: String,
    pub description: String,
    pub date: i64,
    pub link: String,
    pub file: String,
}

pub fn get_recent_posts(limit: usize) -> Vec<Post> {
    let posts: Vec<Post> = serde_json::from_str(
        &std::fs::read_to_string("./client/posts.json").unwrap()
    ).unwrap();

    return posts.into_iter().take(limit).collect();
}

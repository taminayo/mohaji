use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Post {
    title: String,
}

impl Post {
    pub fn new(title: String) -> Self {
        Self { title, }
    }
}
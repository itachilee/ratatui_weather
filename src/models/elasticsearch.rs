use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug)]
pub struct Document {
    pub id: u32,
    pub title: String,
    pub content: String,
}

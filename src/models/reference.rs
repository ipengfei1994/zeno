use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Reference {
    pub url: String,
    pub title: Option<String>,
    pub accessed_at: chrono::DateTime<chrono::Utc>,
}

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Task {
    pub id: String,
    pub title: String,
    #[serde(default = "default_status")]
    pub status: String,
    pub description: String,
}

fn default_status() -> String {
    "todo".to_string()
}

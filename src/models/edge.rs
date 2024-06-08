use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Edge {
    pub r#in: String,
    pub out: String,
}

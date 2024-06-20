use async_graphql::SimpleObject;
use serde::{Deserialize, Serialize};

use crate::graphql::types::surreal_id::SurrealID;

#[derive(SimpleObject, Serialize, Deserialize, Debug)]
pub struct InterestRelation {
    pub id: SurrealID,
    pub r#in: String,
    pub out: String,
}

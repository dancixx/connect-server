use async_graphql::SimpleObject;
use serde::{Deserialize, Serialize};

use crate::graphql::types::{surreal_datetime::SurrealDateTime, surreal_id::SurrealID};

#[derive(SimpleObject, Serialize, Deserialize, Debug)]
pub struct Chat {
    pub id: SurrealID,
    #[graphql(name = "created_at")]
    pub created_at: SurrealDateTime,
    #[graphql(name = "is_deleted")]
    pub is_deleted: bool,
    #[graphql(name = "is_read")]
    pub is_read: bool,
    pub message: Option<String>,
    pub reaction: Option<String>,
    #[graphql(name = "read_at")]
    pub read_at: Option<SurrealDateTime>,
    #[graphql(name = "updated_at")]
    pub updated_at: SurrealDateTime,
}

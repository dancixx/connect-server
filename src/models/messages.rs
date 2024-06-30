use async_graphql::{InputObject, SimpleObject};
use serde::{Deserialize, Serialize};

use crate::graphql::types::{surreal_datetime::SurrealDateTime, surreal_id::SurrealID};

use super::users::User;

#[derive(SimpleObject, Serialize, Deserialize, Debug, Clone)]
pub struct Message {
    pub id: SurrealID,
    pub r#in: SurrealID,
    pub out: SurrealID,
    pub text: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reaction: Option<String>,

    #[graphql(name = "is_read")]
    pub is_read: bool,

    #[graphql(name = "is_deleted")]
    pub is_deleted: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[graphql(name = "read_at")]
    pub read_at: Option<SurrealDateTime>,

    #[graphql(name = "created_at")]
    pub created_at: SurrealDateTime,

    #[graphql(name = "updated_at")]
    pub updated_at: SurrealDateTime,

    pub user: Box<User>,
}

#[derive(InputObject, Serialize, Deserialize, Debug, Default)]
#[graphql(name = "chats_update_set_input")]
pub struct UpdateSetInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,

    #[graphql(name = "is_deleted")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_deleted: Option<bool>,

    #[graphql(name = "is_reported")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_reported: Option<bool>,

    #[graphql(name = "is_read")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_read: Option<bool>,

    #[graphql(name = "read_at")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_at: Option<SurrealDateTime>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reaction: Option<String>,
}

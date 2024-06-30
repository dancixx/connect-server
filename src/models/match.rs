use async_graphql::SimpleObject;
use serde::{Deserialize, Serialize};

use crate::graphql::types::{surreal_datetime::SurrealDateTime, surreal_id::SurrealID};

use super::users;

#[derive(SimpleObject, Serialize, Deserialize, Debug)]
pub struct Match {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<SurrealID>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#in: Option<Box<users::User>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub out: Option<Box<users::User>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[graphql(name = "in_swipe")]
    pub in_swipe: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[graphql(name = "out_swipe")]
    pub out_swipe: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[graphql(name = "is_super_swipe")]
    pub is_super_swipe: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[graphql(name = "is_unmatched")]
    pub is_unmatched: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[graphql(name = "unmatched_at")]
    pub unmatched_at: Option<SurrealDateTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[graphql(name = "message_count")]
    pub message_count: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[graphql(name = "unread_message_count_by_in")]
    pub unread_message_count_by_in: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[graphql(name = "unread_message_count_by_out")]
    pub unread_message_count_by_out: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[graphql(name = "chat_disabled_by_in")]
    pub chat_disabled_by_in: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[graphql(name = "chat_disabled_by_out")]
    pub chat_disabled_by_out: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[graphql(name = "created_at")]
    pub created_at: Option<SurrealDateTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[graphql(name = "updated_at")]
    pub updated_at: Option<SurrealDateTime>,
}

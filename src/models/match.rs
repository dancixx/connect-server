use async_graphql::{ComplexObject, Context, FieldResult, SimpleObject};
use serde::{Deserialize, Serialize};
use surrealdb::method::Content;

use crate::graphql::types::{
    surreal_datetime::SurrealDateTime, surreal_id::SurrealID, SurrealClient,
};

use super::{messages::Message, users};

#[derive(SimpleObject, Serialize, Deserialize, Debug)]
#[graphql(complex)]
pub struct Match {
    pub id: SurrealID,
    pub r#in: Box<users::User>,
    pub out: Box<users::User>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[graphql(name = "in_swipe")]
    pub in_swipe: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[graphql(name = "out_swipe")]
    pub out_swipe: Option<bool>,

    #[graphql(name = "is_super_swipe")]
    pub is_super_swipe: Option<bool>,

    #[graphql(name = "is_unmatched")]
    pub is_unmatched: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[graphql(name = "unmatched_at")]
    pub unmatched_at: Option<SurrealDateTime>,

    #[graphql(name = "message_count")]
    pub message_count: Option<usize>,

    #[graphql(name = "unread_message_count_by_in")]
    pub unread_message_count_by_in: Option<usize>,

    #[graphql(name = "unread_message_count_by_out")]
    pub unread_message_count_by_out: Option<usize>,

    #[graphql(name = "disabled_by_in")]
    pub disabled_by_in: bool,

    #[graphql(name = "disabled_by_out")]
    pub disabled_by_out: bool,

    #[graphql(name = "created_at")]
    pub created_at: SurrealDateTime,

    #[graphql(name = "updated_at")]
    pub updated_at: SurrealDateTime,
}

#[ComplexObject]
impl Match {
    #[graphql(name = "last_message")]
    pub async fn last_message(&self, context: &Context<'_>) -> FieldResult<Option<Message>> {
        let surreal = context.data::<SurrealClient>()?;

        todo!()
    }
}

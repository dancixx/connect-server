use async_graphql::{Context, FieldResult, Object};
use serde_json::json;

use crate::{
    graphql::types::{surreal_id::SurrealID, SurrealClient},
    models::chats,
};

#[derive(Default)]
pub struct ChatsMutationRoot;

#[Object]
impl ChatsMutationRoot {
    #[graphql(name = "insert_chat_message_one")]
    async fn insert_chat_message_one(
        &self,
        context: &Context<'_>,
        #[graphql(name = "user_id")] user_id: String,
        #[graphql(name = "target_user_id")] target_user_id: String,
        message: String,
    ) -> FieldResult<String> {
        let surreal = context.data::<SurrealClient>()?;
        let query = format!(
            r#"
            LET $in = {user_id};
            LET $out = {target_user_id};
            LET $chat_id = SELECT id FROM user_edge WHERE (in = {user_id} && out = {target_user_id}) || (in = {target_user_id} && out = {user_id});
            LET $message_id = (RELATE $chat_id->chat_edge->(INSERT INTO chat_message {{message: "{message}"}})).out;
            RELATE $message_id->chat_message_user_edge->$in;
            "#,
            user_id = user_id,
            target_user_id = target_user_id,
            message = message
        );
        let query = surreal.query(query).await;

        if let Err(e) = query {
            tracing::error!("Error: {:?}", e);
            return Err(e.into());
        }

        Ok(String::from("Chat message inserted"))
    }

    #[graphql(name = "update_chat_message_by_pk")]
    async fn update_chat_message_by_pk(
        &self,
        context: &Context<'_>,
        id: String,
        #[graphql(name = "_set")] _set: chats::UpdateSetInput,
    ) -> FieldResult<String> {
        let surreal = context.data::<SurrealClient>()?;
        let SurrealID(thing) = SurrealID::from(id);

        surreal
            .update::<Option<chats::Chat>>(&thing)
            .merge(_set)
            .await?;

        Ok(String::from("Chat message updated"))
    }

    #[graphql(name = "delete_chat_message_by_pk")]
    async fn delete_chat_message_by_pk(
        &self,
        context: &Context<'_>,
        id: String,
    ) -> FieldResult<String> {
        let surreal = context.data::<SurrealClient>()?;
        let SurrealID(thing) = SurrealID::from(id);
        surreal.delete::<Option<chats::Chat>>(thing).await?;

        Ok(String::from("Chat message deleted"))
    }

    #[graphql(name = "update_chat_message_many")]
    async fn update_chat_message_many(
        &self,
        context: &Context<'_>,
        ids: Vec<String>,
        #[graphql(name = "_set")] _set: chats::UpdateSetInput,
    ) -> FieldResult<String> {
        let surreal = context.data::<SurrealClient>()?;
        let query = format!(
            "UPDATE chat_message MERGE {_set} WHERE id IN {ids:?};",
            _set = json!(_set),
        );
        surreal.query(query).await?;
        Ok(String::from("Chat messages updated"))
    }
}

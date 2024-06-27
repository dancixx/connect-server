use async_graphql::{Context, FieldResult, Object};
use serde_json::json;

use crate::{
    graphql::types::{surreal_id::SurrealID, SurrealClient},
    models::messages::{self, UpdateSetInput},
};

#[derive(Default)]
pub struct MessageMutationRoot;

#[Object]
impl MessageMutationRoot {
    #[graphql(name = "insert_message_one")]
    async fn insert_message_one(
        &self,
        context: &Context<'_>,
        #[graphql(name = "user_id")] user_id: String,
        #[graphql(name = "target_user_id")] target_user_id: String,
        text: String,
    ) -> FieldResult<String> {
        let surreal = context.data::<SurrealClient>()?;
        let query = format!(
            r#"
            LET $in = {user_id};
            LET $out = {target_user_id};
            LET $match_id = SELECT id FROM match WHERE (in = {user_id} && out = {target_user_id}) || (in = {target_user_id} && out = {user_id});
            RELATE $match_id->message->{user_id} SET text = "{text}";
            "#,
            user_id = user_id,
            target_user_id = target_user_id,
            text = text
        );
        let query = surreal.query(query).await;

        if let Err(e) = query {
            tracing::error!("Error: {:?}", e);
            return Err(e.into());
        }

        Ok(String::from("Chat message inserted"))
    }

    #[graphql(name = "update_message_by_pk")]
    async fn update_message_by_pk(
        &self,
        context: &Context<'_>,
        id: String,
        #[graphql(name = "_set")] _set: messages::UpdateSetInput,
    ) -> FieldResult<String> {
        let surreal = context.data::<SurrealClient>()?;
        let SurrealID(thing) = SurrealID::from(id);

        surreal
            .update::<Option<messages::Message>>(&thing)
            .merge(_set)
            .await?;

        Ok(String::from("Chat message updated"))
    }

    #[graphql(name = "delete_message_by_pk")]
    async fn delete_message_by_pk(&self, context: &Context<'_>, id: String) -> FieldResult<String> {
        let surreal = context.data::<SurrealClient>()?;
        let SurrealID(thing) = SurrealID::from(id);
        // This is not a real delete, but a soft delete
        surreal
            .update::<Option<messages::Message>>(thing)
            .merge(UpdateSetInput {
                is_deleted: Some(true),
                ..Default::default()
            })
            .await?;

        Ok(String::from("Chat message deleted"))
    }

    #[graphql(name = "update_message_many")]
    async fn update_message_many(
        &self,
        context: &Context<'_>,
        ids: Vec<String>,
        #[graphql(name = "_set")] _set: messages::UpdateSetInput,
    ) -> FieldResult<String> {
        let surreal = context.data::<SurrealClient>()?;
        let query = format!(
            "UPDATE message MERGE {_set} WHERE id ∈ {ids:?};",
            _set = json!(_set),
        );
        surreal.query(query).await?;
        Ok(String::from("Chat messages updated"))
    }
}

use async_graphql::{Context, FieldResult, Object};

use crate::{graphql::types::SurrealClient, models::chats};

#[derive(Default)]
pub struct ChatsQueryRoot;

#[Object]
impl ChatsQueryRoot {
    #[graphql(name = "select_chat_by_users")]
    async fn select_chat_by_users(
        &self,
        context: &Context<'_>,
        #[graphql(name = "user_id")] user_id: String,
        #[graphql(name = "target_user_id")] target_user_id: String,
    ) -> FieldResult<Vec<chats::Chat>> {
        let surreal = context.data::<SurrealClient>()?;
        // TODO: add pagination
        let query = format!(
            "
            LET $chat_id = SELECT id FROM match WHERE (in = {user_id} && out = {target_user_id}) || (in = {target_user_id} && out = {user_id});
            LET $chat_id = array::first($chat_id);
            SELECT *, array::first(->chat_message_user_edge->user.*) as user FROM $chat_id->chat_edge->chat_message.* ORDER BY created_at DESC;
            ",
            user_id = user_id,
            target_user_id = target_user_id
        );

        let query = surreal.query(query).await;
        if let Err(e) = query {
            tracing::error!("Error: {:?}", e);
            return Err(e.into());
        }

        Ok(query?.take::<Vec<chats::Chat>>(2)?)
    }
}

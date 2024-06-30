use async_graphql::{Context, FieldResult, Object};

use crate::{graphql::types::SurrealClient, models::messages};

#[derive(Default)]
pub struct MessageQueryRoot;

#[Object]
impl MessageQueryRoot {
    #[graphql(name = "select_chat_by_users")]
    async fn select_chat_by_users(
        &self,
        context: &Context<'_>,
        #[graphql(name = "user_id")] user_id: String,
        #[graphql(name = "target_user_id")] target_user_id: String,
    ) -> FieldResult<Vec<messages::Message>> {
        let surreal = context.data::<SurrealClient>()?;
        // TODO: add pagination
        let query = format!(
            "
            LET $chat = SELECT id FROM match WHERE (in = {0} && out = {1}) || (in = {1} && out = {0});
            LET $match_id = array::first($chat).id;
            SELECT *, out.* as user FROM message WHERE in = $match_id ORDER BY created_at DESC;
            ",
            user_id,
            target_user_id
        );
        let query = surreal.query(query).await;

        if let Err(e) = query {
            tracing::error!("Error: {:?}", e);
            return Err(e.into());
        }

        Ok(query?.take::<Vec<messages::Message>>(2)?)
    }
}

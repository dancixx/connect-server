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
            LET $match_id = SELECT id FROM match WHERE (in = {user_id} && out = {target_user_id}) || (in = {target_user_id} && out = {user_id});
            LET $match_id = array::first($match_id);
            SELECT *, out.* as user FROM $match_id->message ORDER BY created_at DESC;
            ",
            user_id = user_id,
            target_user_id = target_user_id
        );

        let query = surreal.query(query).await;
        if let Err(e) = query {
            tracing::error!("Error: {:?}", e);
            return Err(e.into());
        }

        Ok(query?.take::<Vec<messages::Message>>(2)?)
    }
}

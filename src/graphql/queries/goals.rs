use async_graphql::{Context, FieldResult, Object};
use surrealdb::{engine::remote::ws::Client, Surreal};

use crate::models::i18n::I18n;

#[derive(Default)]
pub struct GoalsQueryRoot;

#[Object]
impl GoalsQueryRoot {
    #[graphql(name = "select_goals")]
    async fn select_goals(
        &self,
        context: &Context<'_>,
        #[graphql(name = "order_key", default = "en")] order_key: String,
    ) -> FieldResult<Vec<I18n>> {
        let surreal = context.data::<Surreal<Client>>()?;
        let query = format!("SELECT * FROM goal ORDER BY {}", order_key);
        let query = surreal.query(query).await;

        if let Err(e) = query {
            tracing::error!("Error: {:?}", e);
            return Err(e.into());
        }

        let goals = query?.take::<Vec<I18n>>(0)?;
        Ok(goals)
    }
}

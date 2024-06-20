use async_graphql::{Context, FieldResult, Object};
use surrealdb::{engine::remote::ws::Client, Surreal};

use crate::models::i18n::I18n;

#[derive(Default)]
pub struct GoalsQueryRoot;

#[Object]
impl GoalsQueryRoot {
    #[graphql(name = "select_goals")]
    async fn select_goals(&self, context: &Context<'_>) -> FieldResult<Vec<I18n>> {
        let surreal = context.data::<Surreal<Client>>()?;
        let goals = surreal.select("goal").await.unwrap();
        Ok(goals)
    }
}

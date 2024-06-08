use async_graphql::{Context, FieldResult, Object};
use surrealdb::{engine::remote::ws::Client, Surreal};

use crate::models::goals::Goal;

#[derive(Default)]
pub struct GoalsQueryRoot;

#[Object]
impl GoalsQueryRoot {
    #[graphql(name = "select_goals")]
    async fn select_goals(&self, context: &Context<'_>) -> FieldResult<Vec<Goal>> {
        let surreal = context.data::<Surreal<Client>>()?;
        let goals = surreal.select::<Vec<Goal>>("goals").await.unwrap();
        Ok(goals)
    }
}

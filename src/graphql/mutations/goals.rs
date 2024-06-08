use async_graphql::{Context, FieldResult, Object};
use surrealdb::{engine::remote::ws::Client, Surreal};

use crate::models::edge::Edge;

#[derive(Default)]
pub struct GoalsMutationRoot;

#[Object]
impl GoalsMutationRoot {
    #[graphql(name = "insert_goals_one")]
    pub async fn insert_goals_one(
        &self,
        context: &Context<'_>,
        #[graphql(name = "user_id")] user_id: String,
        #[graphql(name = "goal_id")] goal_id: String,
    ) -> FieldResult<String> {
        let surreal = context.data::<Surreal<Client>>()?;
        surreal
            .insert::<Vec<Edge>>("goals_relations")
            .content(vec![Edge {
                r#in: user_id,
                out: goal_id,
            }])
            .await
            .unwrap();
        Ok(String::from("Inserted"))
    }
}

use async_graphql::{Context, FieldResult, Object};
use surrealdb::{engine::remote::ws::Client, Surreal};

use crate::models::interests::Interest;

#[derive(Default)]
pub struct InterestsQueryRoot;

#[Object]
impl InterestsQueryRoot {
    #[graphql(name = "select_interests")]
    async fn select_interests(&self, context: &Context<'_>) -> FieldResult<Vec<Interest>> {
        let surreal = context.data::<Surreal<Client>>()?;
        let interests = surreal.select::<Vec<Interest>>("interest").await.unwrap();
        Ok(interests)
    }
}

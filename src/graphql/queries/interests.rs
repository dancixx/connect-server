use async_graphql::{Context, FieldResult, Object};
use surrealdb::{engine::remote::ws::Client, Surreal};

use crate::models::i18n::I18n;

#[derive(Default)]
pub struct InterestsQueryRoot;

#[Object]
impl InterestsQueryRoot {
    #[graphql(name = "select_interests")]
    async fn select_interests(&self, context: &Context<'_>) -> FieldResult<Vec<I18n>> {
        let surreal = context.data::<Surreal<Client>>()?;
        let interests = surreal.select("interest").await.unwrap();
        Ok(interests)
    }
}

use async_graphql::{Context, FieldResult, Object};
use surrealdb::{engine::remote::ws::Client, Surreal};

use crate::{graphql::types::surreal_id::SurrealID, models::users};

#[derive(Default)]
pub struct UsersQueryRoot;

#[Object]
impl UsersQueryRoot {
    #[graphql(name = "select_users")]
    async fn select_users(&self, context: &Context<'_>) -> FieldResult<Vec<users::User>> {
        let surreal = context.data::<Surreal<Client>>()?;
        let users = surreal.select::<Vec<users::User>>("users").await?;
        Ok(users)
    }

    #[graphql(name = "select_users_by_id")]
    async fn select_users_by_id(
        &self,
        context: &Context<'_>,
        id: String,
    ) -> FieldResult<Option<users::User>> {
        let surreal = context.data::<Surreal<Client>>()?;
        let SurrealID(thing) = SurrealID::from(id);
        let user = surreal
            .select::<Option<users::User>>(("users", thing.id))
            .await?;
        Ok(user)
    }
}

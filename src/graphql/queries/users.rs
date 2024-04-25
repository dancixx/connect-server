use async_graphql::{Context, FieldResult, Object};
use surrealdb::{engine::remote::ws::Client, Surreal};

use crate::models::users;

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
}

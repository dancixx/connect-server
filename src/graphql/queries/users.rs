use async_graphql::{Context, FieldResult, Object};
use surrealdb::{engine::remote::ws::Client, Surreal};

use crate::{graphql::types::surreal_id::SurrealID, models::users};

#[derive(Default)]
pub struct UsersQueryRoot;

#[Object]
impl UsersQueryRoot {
    #[graphql(name = "select_users")]
    async fn select_users(
        &self,
        context: &Context<'_>,
        limit: Option<usize>,
        offset: Option<usize>,
    ) -> FieldResult<Vec<users::User>> {
        let surreal = context.data::<Surreal<Client>>()?;
        let query = surreal
            .query("SELECT * FROM users LIMIT ($limit) START ($offset);")
            .bind(("limit", limit.unwrap_or(100)))
            .bind(("offset", offset.unwrap_or(0)))
            .await;

        if let Err(e) = query {
            tracing::error!("Error: {:?}", e);
            return Err(e.into());
        }

        let users = query?.take::<Vec<users::User>>(0)?;

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
        let user = surreal.select::<Option<users::User>>(thing).await?;
        Ok(user)
    }
}

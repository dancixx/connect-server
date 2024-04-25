use async_graphql::{Context, FieldResult, Object};
use serde_json::json;
use surrealdb::{engine::remote::ws::Client, Surreal};

use crate::models::users;

#[derive(Default)]
pub struct UsersMutationRoot;

#[Object]
impl UsersMutationRoot {
    #[graphql(name = "insert_users_one")]
    async fn insert_users_one(
        &self,
        context: &Context<'_>,
        input: users::InsertInput,
    ) -> FieldResult<users::User> {
        todo!()
    }

    #[graphql(name = "update_users_by_pk")]
    async fn update_users_by_pk(
        &self,
        context: &Context<'_>,
        id: String,
        input: users::UpdateInput,
    ) -> FieldResult<users::User> {
        let surreal = context.data::<Surreal<Client>>()?;
        let user = surreal
            .update::<Option<users::User>>(("users", id))
            .merge(json!(input))
            .await?;
        Ok(user.unwrap())
    }

    #[graphql(name = "delete_users_by_pk")]
    async fn delete_users_by_pk(&self, context: &Context<'_>, id: i32) -> FieldResult<users::User> {
        todo!()
    }
}

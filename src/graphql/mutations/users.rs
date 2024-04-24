use async_graphql::{Context, FieldResult, Object};

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
        id: i32,
        input: users::UpdateInput,
    ) -> FieldResult<users::User> {
        todo!()
    }

    #[graphql(name = "delete_users_by_pk")]
    async fn delete_users_by_pk(&self, context: &Context<'_>, id: i32) -> FieldResult<users::User> {
        todo!()
    }
}

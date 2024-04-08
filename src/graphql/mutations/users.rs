use async_graphql::{Context, FieldResult, Object};
use deadpool_postgres::Pool;
use postgres_from_row::FromRow;

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
        let client = context.data::<Pool>()?.get().await?;
        let query = client
            .query(
                "INSERT INTO users (email, name) VALUES ($1, $2) RETURNING *",
                &[&input.email, &input.name],
            )
            .await?;
        let user = query.first().map(users::User::from_row).unwrap();
        Ok(user)
    }
}

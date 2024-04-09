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
        let query = format!("INSERT INTO users (email, name) VALUES ($1, $2) RETURNING *");
        let result = client.query(&query, &[&input.email, &input.name]).await?;
        let user = result.first().map(users::User::from_row).unwrap();
        Ok(user)
    }

    #[graphql(name = "update_users_by_pk")]
    async fn update_users_by_pk(
        &self,
        context: &Context<'_>,
        id: i32,
        input: users::UpdateInput,
    ) -> FieldResult<users::User> {
        let client = context.data::<Pool>()?.get().await?;
        // TODO: postgres_from_row not support sql level field selection
        // let fields = context
        //     .field()
        //     .selection_set()
        //     .map(|f| f.name())
        //     .collect::<Vec<&str>>()
        //     .join(",");
        let query = format!("UPDATE users SET name = $1 WHERE id = $2 RETURNING *");
        let result = client.query(&query, &[&input.name, &id]).await?;
        let user = result.first().map(users::User::from_row).unwrap();
        Ok(user)
    }

    #[graphql(name = "delete_users_by_pk")]
    async fn delete_users_by_pk(&self, context: &Context<'_>, id: i32) -> FieldResult<users::User> {
        let client = context.data::<Pool>()?.get().await?;
        let query = format!("DELETE FROM users WHERE id = $1 RETURNING *");
        let result = client.query(&query, &[&id]).await?;
        let user = result.first().map(users::User::from_row).unwrap();
        Ok(user)
    }
}

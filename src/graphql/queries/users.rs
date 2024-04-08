use async_graphql::{Context, FieldResult, Object};
use deadpool_postgres::Pool;
use postgres_from_row::FromRow;

use crate::models::users;

#[derive(Default)]
pub struct UsersQueryRoot;

#[Object]
impl UsersQueryRoot {
    #[graphql(name = "select_users")]
    async fn select_users(&self, context: &Context<'_>) -> FieldResult<Vec<users::User>> {
        let client = context.data::<Pool>()?.get().await?;
        let query = client.query("SELECT * FROM users", &[]).await?;
        let users = query.iter().map(users::User::from_row).collect();
        Ok(users)
    }
}

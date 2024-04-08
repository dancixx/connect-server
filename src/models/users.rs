use async_graphql::{InputObject, SimpleObject};

use postgres_from_row::FromRow;
use serde::{Deserialize, Serialize};

use super::Timestampz;

#[derive(SimpleObject, FromRow, Serialize, Deserialize)]
pub struct User {
    #[graphql(name = "id")]
    pub id: i32,
    #[graphql(name = "email")]
    pub email: String,
    #[graphql(name = "name")]
    pub name: String,
    #[graphql(name = "created_at")]
    pub created_at: Timestampz,
    #[graphql(name = "updated_at")]
    pub updated_at: Timestampz,
}

#[derive(InputObject)]
pub struct InsertInput {
    #[graphql(name = "email")]
    pub email: String,
    #[graphql(name = "name")]
    pub name: String,
}

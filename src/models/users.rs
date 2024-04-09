use async_graphql::{InputObject, SimpleObject};

use postgres_from_row::FromRow;
use serde::{Deserialize, Serialize};

use crate::graphql::types::timestamptz::Timestamptz;

#[derive(SimpleObject, FromRow, Serialize, Deserialize)]
pub struct User {
    #[graphql(name = "id")]
    pub id: i32,
    #[graphql(name = "email")]
    pub email: String,
    #[graphql(name = "name")]
    pub name: String,
    #[graphql(name = "created_at")]
    #[from_row(from = "Timestamptz")]
    pub created_at: Timestamptz,
    #[graphql(name = "updated_at")]
    #[from_row(from = "Timestamptz")]
    pub updated_at: Option<Timestamptz>,
}

#[derive(InputObject)]
pub struct InsertInput {
    #[graphql(name = "email")]
    pub email: String,
    #[graphql(name = "name")]
    pub name: String,
}

#[derive(InputObject)]
pub struct UpdateInput {
    #[graphql(name = "name")]
    pub name: String,
}

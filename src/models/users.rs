use async_graphql::SimpleObject;
use postgres_from_row::FromRow;
use serde::{Deserialize, Serialize};

use super::Timestampz;

#[derive(SimpleObject, FromRow, PartialEq, Eq, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub name: String,
    pub created_at: Timestampz,
    pub updated_at: Timestampz,
}

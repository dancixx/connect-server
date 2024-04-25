use async_graphql::{InputObject, SimpleObject};

use serde::{Deserialize, Serialize};

use crate::graphql::types::{surreal_datetime::SurrealDateTime, surreal_id::SurrealID};

#[derive(SimpleObject, Serialize, Deserialize)]
pub struct User {
    #[graphql(name = "id")]
    pub id: SurrealID,
    #[graphql(name = "email")]
    pub email: String,
    #[graphql(name = "name")]
    pub name: String,
    #[graphql(name = "created_at")]
    pub created_at: SurrealDateTime,
    #[graphql(name = "updated_at")]
    pub updated_at: SurrealDateTime,
}

#[derive(InputObject, Serialize)]
pub struct InsertInput {
    #[graphql(name = "email")]
    pub email: String,
    #[graphql(name = "name")]
    pub name: String,
}

#[derive(InputObject, Serialize)]
pub struct UpdateSetInput {
    #[graphql(name = "name")]
    pub name: String,
}

use async_graphql::{InputObject, SimpleObject};

use serde::{Deserialize, Serialize};

use crate::graphql::types::{surreal_datetime::SurrealDateTime, surreal_id::SurrealID};

#[derive(SimpleObject, Serialize, Deserialize, Default, Debug)]
pub struct User {
    #[graphql(name = "id")]
    pub id: SurrealID,
    #[graphql(name = "email")]
    pub email: String,
    #[graphql(name = "name")]
    pub name: Option<String>,
    #[graphql(name = "last_name")]
    pub last_name: Option<String>,
    #[graphql(name = "first_name")]
    pub first_name: Option<String>,
    #[graphql(name = "created_at")]
    pub created_at: SurrealDateTime,
    #[graphql(name = "updated_at")]
    pub updated_at: SurrealDateTime,
}

#[derive(InputObject, Serialize)]
#[graphql(name = "users_insert_input")]
pub struct InsertInput {
    #[graphql(name = "email")]
    pub email: String,
    #[graphql(name = "name")]
    pub name: Option<String>,
}

#[derive(InputObject, Serialize)]
#[graphql(name = "users_update_set_input")]
pub struct UpdateSetInput {
    #[graphql(name = "name")]
    pub name: String,
}

use async_graphql::scalar;
use serde::{Deserialize, Serialize};
use surrealdb::sql::Datetime;

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct SurrealDateTime(Option<Datetime>);

scalar!(SurrealDateTime);

use async_graphql::scalar;
use serde::{Deserialize, Serialize};
use surrealdb::sql::Datetime;

#[derive(Serialize, Deserialize, Debug)]
pub struct SurrealDateTime(Option<Datetime>);

scalar!(SurrealDateTime, "SurrealDateTime", "Date");

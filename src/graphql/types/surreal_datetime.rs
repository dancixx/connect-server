use async_graphql::scalar;
use serde::{Deserialize, Serialize};
use surrealdb::sql::Datetime;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SurrealDateTime(Option<Datetime>);

scalar!(SurrealDateTime, "SurrealDateTime", "Date");

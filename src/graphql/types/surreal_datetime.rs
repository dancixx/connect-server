use async_graphql::scalar;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct SurrealDateTime(Option<DateTime<Utc>>);

scalar!(SurrealDateTime);

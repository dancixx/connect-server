use async_graphql::scalar;
use geo_types::Point;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct SurrealPoint(pub Point);

scalar!(SurrealPoint);

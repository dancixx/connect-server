use async_graphql::scalar;
use geo_types::Point;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct SurrealPoint(pub Point);

scalar!(
    SurrealPoint,
    "SurrealPoint",
    "This is a scalar type for a point in a 2D plane."
);

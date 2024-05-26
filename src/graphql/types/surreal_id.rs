use async_graphql::{Scalar, ScalarType};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Serialize, Deserialize)]
pub struct SurrealID(Thing);

#[Scalar]
impl ScalarType for SurrealID {
    fn parse(value: async_graphql::Value) -> async_graphql::InputValueResult<Self> {
        if let async_graphql::Value::String(value) = value {
            let value = value.split(':').collect::<Vec<&str>>();
            let thing = Thing::from((value[0], value[1]));
            Ok(Self(thing))
        } else {
            Err(async_graphql::InputValueError::expected_type(value))
        }
    }

    fn to_value(&self) -> async_graphql::Value {
        async_graphql::Value::String(self.0.to_string())
    }
}

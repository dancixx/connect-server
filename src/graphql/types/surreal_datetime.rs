use async_graphql::{InputValueError, InputValueResult, Scalar, ScalarType, Value};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct SurrealDateTime(Option<DateTime<Utc>>);

#[Scalar]
impl ScalarType for SurrealDateTime {
    fn parse(value: Value) -> InputValueResult<Self> {
        if let Value::String(value) = &value {
            Ok(Self(Some(
                DateTime::parse_from_rfc3339(value).unwrap().into(),
            )))
        } else {
            Err(InputValueError::expected_type(value))
        }
    }

    fn to_value(&self) -> Value {
        if self.0.is_none() {
            return Value::Null;
        }
        Value::String(self.0.unwrap().to_rfc3339())
    }
}

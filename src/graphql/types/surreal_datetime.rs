use std::borrow::Cow;

use async_graphql::{Scalar, ScalarType, TypeName};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Datetime;

#[derive(Serialize, Deserialize, Debug)]
pub struct SurrealDateTime(Option<Datetime>);

impl TypeName for SurrealDateTime {
    fn type_name() -> Cow<'static, str> {
        Cow::Borrowed("Date")
    }
}

#[Scalar(name = "SurrealDateTime", name_type = true)]
impl ScalarType for SurrealDateTime {
    fn parse(value: async_graphql::Value) -> async_graphql::InputValueResult<Self> {
        if let async_graphql::Value::String(value) = value {
            let datetime = chrono::DateTime::parse_from_rfc3339(&value)?.to_utc();
            let datetime = Datetime::from(datetime);
            Ok(Self(Some(datetime)))
        } else {
            Err(async_graphql::InputValueError::expected_type(value))
        }
    }

    fn to_value(&self) -> async_graphql::Value {
        async_graphql::Value::String(
            self.0
                .as_ref()
                .map(|datetime| datetime.to_string())
                .unwrap_or_default(),
        )
    }
}

use std::error::Error;

use async_graphql::{InputValueError, InputValueResult, Scalar, ScalarType, Value};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tokio_postgres::types::{FromSql, Type};

#[derive(Serialize, Deserialize)]
pub struct Timestamptz(Option<DateTime<Utc>>);

#[Scalar]
impl ScalarType for Timestamptz {
    fn parse(value: Value) -> InputValueResult<Self> {
        if let Value::String(value) = &value {
            Ok(Timestamptz(Some(
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

impl<'a> FromSql<'a> for Timestamptz {
    fn from_sql(_ty: &Type, raw: &'a [u8]) -> Result<Self, Box<dyn Error + 'static + Send + Sync>> {
        let datetime: DateTime<chrono::Utc> = FromSql::from_sql(&Type::TIMESTAMPTZ, raw)?;
        Ok(Timestamptz(Some(datetime)))
    }

    fn accepts(ty: &Type) -> bool {
        ty.name() == "timestamptz"
    }

    fn from_sql_null(_ty: &Type) -> Result<Self, Box<dyn Error + 'static + Sync + Send>> {
        Ok(Timestamptz(None))
    }
}

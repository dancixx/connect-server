use std::{borrow::Cow, ops::Deref};

use async_graphql::{Scalar, ScalarType, TypeName};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Serialize, Deserialize, Debug)]
pub struct SurrealID(pub Thing);

impl From<String> for SurrealID {
    fn from(id: String) -> Self {
        let id = id.split(':').collect::<Vec<&str>>();
        Self(Thing::from((id[0], id[1])))
    }
}

impl Deref for SurrealID {
    type Target = Thing;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl TypeName for SurrealID {
    fn type_name() -> Cow<'static, str> {
        Cow::Borrowed("String")
    }
}

#[Scalar(name = "SurrealID", name_type = true)]
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

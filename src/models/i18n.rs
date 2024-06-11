use async_graphql::SimpleObject;
use serde::{Deserialize, Serialize};

use crate::graphql::types::surreal_id::SurrealID;

#[derive(SimpleObject, Serialize, Deserialize, Debug)]
pub struct I18n {
    pub id: SurrealID,
    pub en: String,
}

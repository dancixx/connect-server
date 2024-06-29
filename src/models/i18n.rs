use async_graphql::SimpleObject;
use serde::{Deserialize, Serialize};

use crate::graphql::types::surreal_id::SurrealID;

#[derive(SimpleObject, Serialize, Deserialize, Debug, Default)]
pub struct I18n {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<SurrealID>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub en: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub de: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub es: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fr: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub it: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ja: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zh: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hu: Option<String>,
}

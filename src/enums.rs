use async_graphql::Enum;
use serde::{Deserialize, Serialize};

#[derive(Enum, Copy, Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub enum Gender {
    Male,
    Female,
    Unisex,
}

use async_graphql::{InputObject, SimpleObject};

use serde::{Deserialize, Serialize};

use crate::{
    enums::Gender,
    graphql::types::{
        surreal_datetime::SurrealDateTime, surreal_id::SurrealID, surreal_point::SurrealPoint,
    },
};

#[derive(SimpleObject, Serialize, Deserialize, Debug)]
pub struct User {
    pub id: SurrealID,
    pub email: String,
    pub username: Option<String>,
    #[graphql(name = "last_name")]
    pub last_name: Option<String>,
    #[graphql(name = "first_name")]
    pub first_name: Option<String>,
    #[graphql(name = "birth_date")]
    pub birth_date: Option<SurrealDateTime>,
    #[graphql(name = "age")]
    pub age: Option<i32>,
    pub gender: Option<Gender>,
    pub preference: Option<Gender>,
    pub metric: bool,
    pub height: Option<i32>,
    pub weight: Option<i32>,
    pub bio: Option<String>,
    pub interests: Option<Vec<String>>,
    #[graphql(name = "current_location")]
    pub current_location: Option<SurrealPoint>,
    #[graphql(name = "max_distance")]
    pub max_distance: i32,
    #[graphql(name = "age_range")]
    pub age_range: Option<Vec<i32>>,
    #[graphql(name = "max_match_count")]
    pub max_match_count: i32,
    #[graphql(name = "active_hours")]
    pub active_hours: Option<Vec<i32>>,
    #[graphql(name = "has_pet")]
    pub has_pet: bool,
    #[graphql(name = "has_car")]
    pub has_car: bool,
    #[graphql(name = "has_house")]
    pub has_house: bool,
    #[graphql(name = "need_pet")]
    pub need_pet: bool,
    #[graphql(name = "need_car")]
    pub need_car: bool,
    #[graphql(name = "need_house")]
    pub need_house: bool,
    #[graphql(name = "open_for_travel_together")]
    pub open_for_travel_together: bool,
    #[graphql(name = "is_registration_completed")]
    pub is_registration_completed: bool,
    #[graphql(name = "is_paid_travel")]
    pub is_paid_travel: bool,
    #[graphql(name = "is_verified")]
    pub is_verified: bool,
    #[graphql(name = "is_banned")]
    pub is_banned: bool,
    #[graphql(name = "created_at")]
    pub created_at: SurrealDateTime,
    #[graphql(name = "updated_at")]
    pub updated_at: SurrealDateTime,
}

#[derive(InputObject, Serialize)]
#[graphql(name = "users_insert_input")]
pub struct InsertInput {
    pub email: String,
    pub username: Option<String>,
}

#[derive(InputObject, Serialize, Deserialize, Debug)]
#[graphql(name = "users_update_set_input")]
pub struct UpdateSetInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[graphql(name = "last_name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[graphql(name = "first_name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[graphql(name = "birth_date")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub birth_date: Option<SurrealDateTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gender: Option<Gender>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<Gender>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bio: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interest: Option<Vec<String>>,
    #[graphql(name = "max_distance")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_distance: Option<i32>,
    #[graphql(name = "age_range")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub age_range: Option<Vec<i32>>,
    #[graphql(name = "active_hours")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_hours: Option<Vec<i32>>,
    #[graphql(name = "has_pet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_pet: Option<bool>,
    #[graphql(name = "has_car")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_car: Option<bool>,
    #[graphql(name = "has_house")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_house: Option<bool>,
    #[graphql(name = "need_pet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub need_pet: Option<bool>,
    #[graphql(name = "need_car")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub need_car: Option<bool>,
    #[graphql(name = "need_house")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub need_house: Option<bool>,
    #[graphql(name = "open_for_travel_together")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_for_travel_together: Option<bool>,
    #[graphql(name = "is_paid_travel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_paid_travel: Option<bool>,
}

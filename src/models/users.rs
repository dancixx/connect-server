use async_graphql::{InputObject, SimpleObject};

use serde::{Deserialize, Serialize};

use crate::{
    enums::Gender,
    graphql::types::{surreal_datetime::SurrealDateTime, surreal_id::SurrealID},
};

#[derive(SimpleObject, Serialize, Deserialize, Default, Debug)]
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
    #[graphql(name = "target_gender")]
    pub target_gender: Option<Gender>,
    pub metric: Option<String>,
    pub height: Option<i32>,
    pub weight: Option<i32>,
    pub bio: Option<String>,
    pub interest: Option<Vec<String>>,
    pub country: Option<String>,
    pub city: Option<String>,
    #[graphql(name = "max_distance")]
    pub max_distance: i32,
    #[graphql(name = "age_range")]
    pub age_range: Option<Vec<i32>>,
    #[graphql(name = "max_match_count")]
    pub max_match_count: i32,
    #[graphql(name = "max_message_count")]
    pub active_hours: Option<Vec<i32>>,
    #[graphql(name = "has_dog")]
    pub has_dog: bool,
    #[graphql(name = "has_cat")]
    pub has_cat: bool,
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

#[derive(InputObject, Serialize)]
#[graphql(name = "users_update_set_input")]
pub struct UpdateSetInput {
    #[graphql(name = "last_name")]
    pub last_name: Option<String>,
    #[graphql(name = "first_name")]
    pub first_name: Option<String>,
    #[graphql(name = "birth_date")]
    pub birth_date: Option<SurrealDateTime>,
    pub gender: Option<Gender>,
    #[graphql(name = "target_gender")]
    pub target_gender: Option<Gender>,
    pub metric: Option<String>,
    pub height: Option<i32>,
    pub weight: Option<i32>,
    pub bio: Option<String>,
    pub interest: Option<Vec<String>>,
    pub country: Option<String>,
    pub city: Option<String>,
    #[graphql(name = "max_distance")]
    pub max_distance: i32,
    #[graphql(name = "age_range")]
    pub age_range: Option<Vec<i32>>,
    #[graphql(name = "max_message_count")]
    pub active_hours: Option<Vec<i32>>,
    #[graphql(name = "has_dog")]
    pub has_dog: bool,
    #[graphql(name = "has_cat")]
    pub has_cat: bool,
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
    #[graphql(name = "is_paid_travel")]
    pub is_paid_travel: bool,
}

use async_graphql::{ComplexObject, Context, FieldResult, InputObject, SimpleObject};

use serde::{Deserialize, Serialize};
use surrealdb::{engine::remote::ws::Client, Surreal};

use crate::{
    enums::Gender,
    graphql::types::{
        surreal_datetime::SurrealDateTime, surreal_id::SurrealID, surreal_point::SurrealPoint,
    },
};

#[derive(SimpleObject, Serialize, Deserialize, Debug)]
#[graphql(complex)]
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
    pub age: Option<i32>,
    pub gender: Option<Gender>,
    pub preference: Option<Gender>,
    #[graphql(skip)]
    goal: Option<String>,
    #[graphql(skip)]
    interests: Option<Vec<String>>,
    pub metric: bool,
    pub height: Option<f32>,
    pub weight: Option<f32>,
    pub bio: Option<String>,
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
    #[graphql(name = "last_message")]
    pub last_message: Option<super::chats::Chat>,
}

#[ComplexObject]
impl User {
    #[graphql(name = "goal")]
    async fn goal_details(&self, context: &Context<'_>) -> FieldResult<Option<super::i18n::I18n>> {
        let surreal = context.data::<Surreal<Client>>()?;
        let goal = self.goal.clone();

        if self.goal.is_none() {
            return Ok(None);
        }

        let SurrealID(thing) = SurrealID::from(goal.unwrap());
        let goal = surreal.select::<Option<super::i18n::I18n>>(thing).await?;

        Ok(goal)
    }

    #[graphql(name = "interests")]
    async fn interests_details(
        &self,
        context: &Context<'_>,
    ) -> FieldResult<Vec<super::i18n::I18n>> {
        let surreal = context.data::<Surreal<Client>>()?;
        let interests = self.interests.clone();

        if self.interests.is_none() {
            return Ok(vec![]);
        }

        let interests = interests.unwrap();
        let query = format!("SELECT * FROM interests WHERE id ∈ {:?};", interests);

        let query = surreal.query(query).await;

        let interests = query?.take::<Vec<super::i18n::I18n>>(0)?;

        Ok(interests)
    }
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
    pub age: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gender: Option<Gender>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<Gender>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub goal: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interests: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bio: Option<String>,
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
    #[graphql(name = "is_registration_completed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_registration_completed: Option<bool>,
}

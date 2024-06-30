use async_graphql::{ComplexObject, Context, FieldResult, InputObject, SimpleObject};

use serde::{Deserialize, Serialize};

use crate::{
    enums::Gender,
    graphql::types::{
        surreal_datetime::SurrealDateTime, surreal_id::SurrealID, surreal_point::SurrealPoint,
        I18nTables, SurrealClient,
    },
};

#[derive(SimpleObject, Serialize, Deserialize, Debug, Clone)]
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

    #[graphql(skip)]
    education: Option<String>,

    pub job: Option<String>,

    #[graphql(skip)]
    alcohol: Option<String>,

    pub smoking: bool,

    #[graphql(skip)]
    political_views: Option<String>,

    #[graphql(skip)]
    religion: Option<String>,

    #[graphql(skip)]
    spoken_languages: Option<Vec<String>>,

    #[graphql(name = "app_language")]
    pub app_language: Option<String>,

    zodiac: Option<String>,

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

    #[graphql(name = "has_kids")]
    pub has_kids: bool,

    #[graphql(name = "want_kids")]
    pub want_kids: Option<bool>,

    #[graphql(name = "spontaneous_travel")]
    pub spontaneous_travel: bool,

    #[graphql(name = "is_registration_completed")]
    pub is_registration_completed: bool,

    #[graphql(name = "is_verified")]
    pub is_verified: bool,

    #[graphql(name = "is_banned")]
    pub is_banned: bool,

    #[graphql(name = "created_at")]
    pub created_at: SurrealDateTime,

    #[graphql(name = "updated_at")]
    pub updated_at: SurrealDateTime,
}

#[ComplexObject]
impl User {
    #[graphql(name = "goal")]
    async fn goal_details(&self, context: &Context<'_>) -> FieldResult<Option<super::i18n::I18n>> {
        let surreal = context.data::<SurrealClient>()?;
        let goal = self.goal.as_ref();

        if self.goal.is_none() {
            return Ok(None);
        }

        let query = format!(
            "SELECT id,{} FROM {} WHERE id = '{}';",
            self.app_language.as_ref().unwrap_or(&"en".to_string()),
            I18nTables::i18n_goal,
            goal.unwrap()
        );
        let query = surreal.query(query).await;

        if let Err(e) = query {
            tracing::error!("Error: {:?}", e);
            return Err(e.into());
        }

        let goal = query?.take::<Option<super::i18n::I18n>>(0)?;
        Ok(goal)
    }

    #[graphql(name = "interests")]
    async fn interests_details(
        &self,
        context: &Context<'_>,
    ) -> FieldResult<Vec<super::i18n::I18n>> {
        let surreal = context.data::<SurrealClient>()?;
        let interests = self.interests.as_ref();

        if self.interests.is_none() {
            return Ok(vec![]);
        }

        let query = format!(
            "SELECT id,{} FROM {} WHERE id ∈ {:?};",
            self.app_language.as_ref().unwrap_or(&"en".to_string()),
            I18nTables::i18n_interest,
            interests.unwrap()
        );
        let query = surreal.query(query).await;
        let interests = query?.take::<Vec<super::i18n::I18n>>(0)?;

        Ok(interests)
    }

    #[graphql(name = "education")]
    async fn education_details(
        &self,
        context: &Context<'_>,
    ) -> FieldResult<Option<super::i18n::I18n>> {
        let surreal = context.data::<SurrealClient>()?;
        let education = self.education.as_ref();

        if self.education.is_none() {
            return Ok(None);
        }

        let query = format!(
            "SELECT id,{} FROM {} WHERE id = '{}';",
            self.app_language.as_ref().unwrap_or(&"en".to_string()),
            I18nTables::i18n_education,
            education.unwrap()
        );
        let query = surreal.query(query).await;

        if let Err(e) = query {
            tracing::error!("Error: {:?}", e);
            return Err(e.into());
        }

        let education = query?.take::<Option<super::i18n::I18n>>(0)?;
        Ok(education)
    }

    #[graphql(name = "alcohol")]
    async fn alcohol_details(
        &self,
        context: &Context<'_>,
    ) -> FieldResult<Option<super::i18n::I18n>> {
        let surreal = context.data::<SurrealClient>()?;
        let alcohol = self.alcohol.as_ref();

        if self.alcohol.is_none() {
            return Ok(None);
        }

        let query = format!(
            "SELECT id,{} FROM {} WHERE id = '{}';",
            self.app_language.as_ref().unwrap_or(&"en".to_string()),
            I18nTables::i18n_alcohol,
            alcohol.unwrap()
        );
        let query = surreal.query(query).await;

        if let Err(e) = query {
            tracing::error!("Error: {:?}", e);
            return Err(e.into());
        }

        let alcohol = query?.take::<Option<super::i18n::I18n>>(0)?;
        Ok(alcohol)
    }

    #[graphql(name = "political_views")]
    async fn political_views_details(
        &self,
        context: &Context<'_>,
    ) -> FieldResult<Option<super::i18n::I18n>> {
        let surreal = context.data::<SurrealClient>()?;
        let political_views = self.political_views.as_ref();

        if self.political_views.is_none() {
            return Ok(None);
        }

        let query = format!(
            "SELECT id,{} FROM {} WHERE id = '{}';",
            self.app_language.as_ref().unwrap_or(&"en".to_string()),
            I18nTables::i18n_political,
            political_views.unwrap()
        );
        let query = surreal.query(query).await;

        if let Err(e) = query {
            tracing::error!("Error: {:?}", e);
            return Err(e.into());
        }

        let political_views = query?.take::<Option<super::i18n::I18n>>(0)?;
        Ok(political_views)
    }

    #[graphql(name = "religion")]
    async fn religion_details(
        &self,
        context: &Context<'_>,
    ) -> FieldResult<Option<super::i18n::I18n>> {
        let surreal = context.data::<SurrealClient>()?;
        let religion = self.religion.as_ref();

        if self.religion.is_none() {
            return Ok(None);
        }

        let query = format!(
            "SELECT id,{} FROM {} WHERE id = '{}';",
            self.app_language.as_ref().unwrap_or(&"en".to_string()),
            I18nTables::i18n_religion,
            religion.unwrap()
        );
        let query = surreal.query(query).await;

        if let Err(e) = query {
            tracing::error!("Error: {:?}", e);
            return Err(e.into());
        }

        let religion = query?.take::<Option<super::i18n::I18n>>(0)?;
        Ok(religion)
    }

    #[graphql(name = "spoken_languages")]
    async fn spoken_languages_details(
        &self,
        context: &Context<'_>,
    ) -> FieldResult<Vec<super::i18n::I18n>> {
        let surreal = context.data::<SurrealClient>()?;
        let spoken_languages = self.spoken_languages.as_ref();

        if self.spoken_languages.is_none() {
            return Ok(vec![]);
        }

        let query = format!(
            "SELECT id,{} FROM {} WHERE id ∈ {:?};",
            self.app_language.as_ref().unwrap_or(&"en".to_string()),
            I18nTables::i18n_language,
            spoken_languages.unwrap()
        );
        let query = surreal.query(query).await;
        let spoken_languages = query?.take::<Vec<super::i18n::I18n>>(0)?;

        Ok(spoken_languages)
    }

    #[graphql(name = "zodiac")]
    async fn zodiac_details(
        &self,
        context: &Context<'_>,
    ) -> FieldResult<Option<super::i18n::I18n>> {
        let surreal = context.data::<SurrealClient>()?;
        let zodiac = self.zodiac.as_ref();

        if self.zodiac.is_none() {
            return Ok(None);
        }

        let query = format!(
            "SELECT id,{} FROM {} WHERE id = '{}';",
            self.app_language.as_ref().unwrap_or(&"en".to_string()),
            I18nTables::i18n_zodiac,
            zodiac.unwrap()
        );
        let query = surreal.query(query).await;

        if let Err(e) = query {
            tracing::error!("Error: {:?}", e);
            return Err(e.into());
        }

        let zodiac = query?.take::<Option<super::i18n::I18n>>(0)?;
        Ok(zodiac)
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

    #[serde(skip_serializing_if = "Option::is_none")]
    pub education: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub job: Option<String>,

    #[graphql(name = "alcohol_consumption")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alcohol_consumption: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub smoking: Option<bool>,

    #[graphql(name = "political_views")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub political_views: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub religion: Option<String>,

    #[graphql(name = "spoken_languages")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spoken_languages: Option<Vec<String>>,

    #[graphql(name = "app_language")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_language: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub zodiac: Option<String>,

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

    #[graphql(name = "has_kids")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_kids: Option<bool>,

    #[graphql(name = "want_kids")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub want_kids: Option<bool>,

    #[graphql(name = "spontaneous_travel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spontaneous_travel: Option<bool>,

    #[graphql(name = "is_registration_completed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_registration_completed: Option<bool>,
}

use async_graphql::{Context, FieldResult, Object};
use serde_json::json;
use surrealdb::{engine::remote::ws::Client, Surreal};

use crate::{graphql::types::surreal_id::SurrealID, models::users};

#[derive(Default)]
pub struct UsersMutationRoot;

#[Object]
impl UsersMutationRoot {
    #[graphql(name = "insert_users_one")]
    async fn insert_users_one(
        &self,
        context: &Context<'_>,
        id: String,
        input: users::InsertInput,
    ) -> FieldResult<users::User> {
        let surreal = context.data::<Surreal<Client>>()?;
        let SurrealID(thing) = SurrealID::from(id);
        let user = surreal
            .create::<Option<users::User>>(thing)
            .content(input)
            .await?;

        Ok(user.ok_or_else(|| "User not found")?)
    }

    #[graphql(name = "update_users_by_pk")]
    async fn update_users_by_pk(
        &self,
        context: &Context<'_>,
        id: String,
        #[graphql(name = "_set")] _set: users::UpdateSetInput,
    ) -> FieldResult<users::User> {
        let surreal = context.data::<Surreal<Client>>()?;
        let SurrealID(thing) = SurrealID::from(id);
        let user = surreal
            .update::<Option<users::User>>(thing)
            .merge(json!(_set))
            .await?;

        Ok(user.ok_or_else(|| "User not found")?)
    }

    #[graphql(name = "update_users_location_by_pk")]
    async fn update_users_location_by_pk(
        &self,
        context: &Context<'_>,
        id: String,
        coordinates: [f64; 2],
    ) -> FieldResult<String> {
        let surreal = context.data::<Surreal<Client>>()?;

        let query = format!(
            "UPDATE {id} SET current_location = {{type: 'Point', coordinates: {coordinates:?}}}",
        );

        // TODO: change to update
        let response = surreal.query(query).await;

        if response.is_err() {
            return Err(response.unwrap_err().into());
        }

        Ok(String::from("Location updated"))
    }

    #[graphql(name = "delete_users_by_pk")]
    async fn delete_users_by_pk<'a>(
        &self,
        context: &Context<'_>,
        id: String,
    ) -> FieldResult<users::User> {
        let surreal = context.data::<Surreal<Client>>()?;
        let SurrealID(thing) = SurrealID::from(id);
        let user = surreal.delete::<Option<users::User>>(thing).await?;

        Ok(user.ok_or_else(|| "User not found")?)
    }
}

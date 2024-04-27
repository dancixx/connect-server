use async_graphql::{Context, FieldResult, Object};
use itertools::Itertools;
use serde_json::json;
use surrealdb::{engine::remote::ws::Client, Surreal};

use crate::models::users;

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
        let user = surreal
            .create::<Option<users::User>>(("users", id))
            .content(input)
            .await?;
        Ok(user.into_iter().next().unwrap())
    }

    #[graphql(name = "update_users_by_pk")]
    async fn update_users_by_pk(
        &self,
        context: &Context<'_>,
        id: String,
        #[graphql(name = "_set")] _set: users::UpdateSetInput,
    ) -> FieldResult<users::User> {
        let surreal = context.data::<Surreal<Client>>()?;
        let surreal_id = id.split(':').collect_tuple::<(&str, &str)>().unwrap();
        let user = surreal
            .update::<Option<users::User>>(surreal_id)
            .merge(json!(_set))
            .await?;
        Ok(user.unwrap())
    }

    #[graphql(name = "delete_users_by_pk")]
    async fn delete_users_by_pk<'a>(
        &self,
        context: &Context<'_>,
        id: String,
    ) -> FieldResult<users::User> {
        let surreal = context.data::<Surreal<Client>>()?;
        let surreal_id = id.split(':').collect_tuple::<(&str, &str)>().unwrap();
        let user = surreal.delete::<Option<users::User>>(surreal_id).await?;
        Ok(user.unwrap())
    }
}

use async_graphql::{Context, FieldResult, Object};
use surrealdb::{engine::remote::ws::Client, Surreal};

use crate::{graphql::types::surreal_id::SurrealID, models::users};

#[derive(Default)]
pub struct UsersQueryRoot;

#[Object]
impl UsersQueryRoot {
    #[graphql(name = "select_users")]
    async fn select_users(
        &self,
        context: &Context<'_>,
        #[graphql(name = "user_id")] user_id: String,
        #[graphql(default = 20)] limit: i32,
        #[graphql(default = 0)] offset: i32,
    ) -> FieldResult<Vec<users::User>> {
        let surreal = context.data::<Surreal<Client>>()?;
        let query = format!(
            "
                LET $swiped = (array::first((SELECT ->(user_edge WHERE in_swipe = true)->user AS users FROM {0})).users);
                SELECT * FROM user WHERE (id ∉ $swiped && (id != {0})) LIMIT {1} START {2};
            ",
            user_id, limit, offset
        );
        let query = surreal.query(query).await;

        if let Err(e) = query {
            tracing::error!("Error: {:?}", e);
            return Err(e.into());
        }

        let users = query?.take::<Vec<users::User>>(1)?;

        Ok(users)
    }

    #[graphql(name = "select_users_by_id")]
    async fn select_users_by_id(
        &self,
        context: &Context<'_>,
        id: String,
    ) -> FieldResult<Option<users::User>> {
        let surreal = context.data::<Surreal<Client>>()?;
        let SurrealID(thing) = SurrealID::from(id);
        let user = surreal.select::<Option<users::User>>(thing).await?;
        Ok(user)
    }

    #[graphql(name = "select_users_swiped")]
    async fn select_users_swiped(
        &self,
        context: &Context<'_>,
        #[graphql(name = "user_id")] user_id: String,
    ) -> FieldResult<Vec<users::User>> {
        let surreal = context.data::<Surreal<Client>>()?;
        let query = format!(
            "array::first(SELECT ->(user_edge WHERE in_swipe = true)->user.* AS users FROM {}).users;",
            user_id
        );
        let query = surreal.query(query).await;

        if let Err(e) = query {
            tracing::error!("Error: {:?}", e);
            return Err(e.into());
        }

        let users = query?.take::<Vec<users::User>>(0)?;

        Ok(users)
    }

    #[graphql(name = "select_users_swiper")]
    async fn select_users_swiper(
        &self,
        context: &Context<'_>,
        #[graphql(name = "user_id")] user_id: String,
    ) -> FieldResult<Vec<users::User>> {
        let surreal = context.data::<Surreal<Client>>()?;
        let query = format!(
            "array::first(SELECT <-(user_edge WHERE in_swipe = true)<-user.* AS users FROM {}).users;",
            user_id
        );
        let query = surreal.query(query).await;

        if let Err(e) = query {
            tracing::error!("Error: {:?}", e);
            return Err(e.into());
        }

        let users = query?.take::<Vec<users::User>>(0)?;

        Ok(users)
    }
}

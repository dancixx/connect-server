use async_graphql::{Context, FieldResult, Object};

use crate::{
    graphql::types::{surreal_id::SurrealID, SurrealClient},
    models::{r#match, users},
};

#[derive(Default)]
pub struct UserQueryRoot;

#[Object]
impl UserQueryRoot {
    #[graphql(name = "select_users")]
    async fn select_users(
        &self,
        context: &Context<'_>,
        #[graphql(name = "user_id")] user_id: String,
        #[graphql(default = 20)] limit: i32,
        #[graphql(default = 0)] offset: i32,
    ) -> FieldResult<Vec<users::User>> {
        let surreal = context.data::<SurrealClient>()?;
        let query = format!(
            // This query has two parts:
            // 1. Get the users that the current user has swiped on
            // 2. Get the users that the current user has not swiped on
            "
                LET $swiped = (SELECT out FROM match WHERE in = {0}).out;
                LET $match = (SELECT in FROM match WHERE out = {0}).in;
                LET $ids = array::concat($swiped, $match);
                LET $ids = array::add($ids, {0});
                SELECT * FROM user WHERE id ∉ $ids LIMIT {1} START {2};
            ",
            user_id, limit, offset
        );
        let query = surreal.query(query).await;

        if let Err(e) = query {
            tracing::error!("Error: {:?}", e);
            return Err(e.into());
        }

        // This query has two parts and this is the reason why we are using take::<Vec<users::User>>(1)
        let users = query?.take::<Vec<users::User>>(4)?;

        Ok(users)
    }

    #[graphql(name = "select_users_by_id")]
    async fn select_users_by_id(
        &self,
        context: &Context<'_>,
        id: String,
    ) -> FieldResult<Option<users::User>> {
        let surreal = context.data::<SurrealClient>()?;
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
        let surreal = context.data::<SurrealClient>()?;
        let query = format!(
            "array::first(SELECT ->(match WHERE in_swipe = true && out_swipe != true)->user.* AS users FROM {}).users;",
            user_id
        );

        let query = surreal.query(query).await;
        if let Err(e) = query {
            tracing::error!("Error: {:?}", e);
            return Err(e.into());
        }

        Ok(query?.take::<Vec<users::User>>(0)?)
    }

    #[graphql(name = "select_users_swiper")]
    async fn select_users_swiper(
        &self,
        context: &Context<'_>,
        #[graphql(name = "user_id")] user_id: String,
    ) -> FieldResult<Vec<users::User>> {
        let surreal = context.data::<SurrealClient>()?;
        let query = format!(
            "array::first(SELECT <-(match WHERE in_swipe = true && out_swipe != true)<-user.* AS users FROM {}).users;",
            user_id
        );

        let query = surreal.query(query).await;
        if let Err(e) = query {
            tracing::error!("Error: {:?}", e);
            return Err(e.into());
        }

        Ok(query?.take::<Vec<users::User>>(0)?)
    }

    #[graphql(name = "select_users_matches")]
    async fn select_users_matches(
        &self,
        context: &Context<'_>,
        #[graphql(name = "user_id")] user_id: String,
    ) -> FieldResult<Vec<r#match::Match>> {
        let surreal = context.data::<SurrealClient>()?;
        // TODO: improve this query
        // TODO: order by created_at
        let query = format!(
            "SELECT *, in.*, out.* FROM match WHERE (in = {0} || out = {0}) && (in_swipe = true && out_swipe = true) ORDER BY created_at DESC;",
            user_id
        );

        let query = surreal.query(query).await;
        if let Err(e) = query {
            tracing::error!("Error: {:?}", e);
            return Err(e.into());
        }

        Ok(query?.take::<Vec<r#match::Match>>(0)?)
    }
}

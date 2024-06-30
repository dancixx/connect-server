use async_graphql::{Context, FieldResult, Object};

use crate::{
    graphql::types::{surreal_id::SurrealID, SurrealClient},
    models::{r#match::Match, users},
};

#[derive(Default)]
pub struct UserMutationRoot;

#[Object]
impl UserMutationRoot {
    #[graphql(name = "insert_users_one")]
    async fn insert_users_one(
        &self,
        context: &Context<'_>,
        id: String,
        input: users::InsertInput,
    ) -> FieldResult<users::User> {
        let surreal = context.data::<SurrealClient>()?;
        let SurrealID(thing) = SurrealID::from(id);
        let user = surreal
            .create::<Option<users::User>>(thing)
            .content(input)
            .await?;

        Ok(user.ok_or("User not found")?)
    }

    #[graphql(name = "update_users_by_pk")]
    async fn update_users_by_pk(
        &self,
        context: &Context<'_>,
        id: String,
        #[graphql(name = "_set")] _set: users::UpdateSetInput,
    ) -> FieldResult<users::User> {
        let surreal = context.data::<SurrealClient>()?;
        let SurrealID(thing) = SurrealID::from(id);

        let user = surreal
            .update::<Option<users::User>>(&thing)
            .merge(_set)
            .await?;

        Ok(user.ok_or("User not found")?)
    }

    #[graphql(name = "update_users_location_by_pk")]
    async fn update_users_location_by_pk(
        &self,
        context: &Context<'_>,
        id: String,
        coordinates: [f64; 2],
    ) -> FieldResult<String> {
        let surreal = context.data::<SurrealClient>()?;

        let query = format!(
            "UPDATE {id} SET current_location = {{type: 'Point', coordinates: {coordinates:?}}}",
        );

        let response = surreal.query(query).await;
        if let Err(e) = response {
            return Err(e.into());
        }

        Ok(String::from("Location updated"))
    }

    #[graphql(name = "delete_users_by_pk")]
    async fn delete_users_by_pk<'a>(
        &self,
        context: &Context<'_>,
        id: String,
    ) -> FieldResult<String> {
        let surreal = context.data::<SurrealClient>()?;
        let SurrealID(thing) = SurrealID::from(id);
        surreal.delete::<Option<users::User>>(thing).await?;

        Ok(String::from("User deleted"))
    }

    #[graphql(name = "swipe_users_right_by_pk")]
    async fn swipe_users_right_by_pk(
        &self,
        context: &Context<'_>,
        #[graphql(name = "user_id")] user_id: String,
        #[graphql(name = "target_user_id")] target_user_id: String,
    ) -> FieldResult<String> {
        let surreal = context.data::<SurrealClient>()?;
        tracing::info!("User ID: {:?}", user_id);
        tracing::info!("Target User ID: {:?}", target_user_id);
        let query = format!(
            "array::first(SELECT * FROM match WHERE in = {target_user_id} && out = {user_id});",
            user_id = user_id
        );

        let mut response = surreal.query(query).await?;
        let response = response.take::<Option<Match>>(0)?;
        tracing::info!("Match: {:?}", response);
        let q: String;

        if let Some(r#match) = response {
            q = format!(
                "UPDATE {match_id} SET out_swipe = true;",
                match_id = *r#match.id
            );
        } else {
            q = format!(
                "RELATE {user_id}->match->{user_target_id} SET in_swipe = true",
                user_id = user_id,
                user_target_id = target_user_id
            )
        }
        surreal.query(q).await?;

        Ok(String::from("User swiped right"))
    }

    #[graphql(name = "swipe_users_left_by_pk")]
    async fn swipe_users_left_by_pk(
        &self,
        context: &Context<'_>,
        #[graphql(name = "user_id")] user_id: String,
        #[graphql(name = "target_user_id")] target_user_id: String,
    ) -> FieldResult<String> {
        let surreal = context.data::<SurrealClient>()?;
        let relate = format!(
            "RELATE {user_id}->match->{target_user_id} SET in_swipe = false",
            user_id = user_id,
            target_user_id = target_user_id
        );
        surreal.query(relate).await?;

        Ok(String::from("User swiped left"))
    }

    #[graphql(name = "swipe_users_up_by_pk")]
    async fn swipe_users_up_by_pk(
        &self,
        context: &Context<'_>,
        #[graphql(name = "user_id")] user_id: String,
        #[graphql(name = "target_user_id")] target_user_id: String,
    ) -> FieldResult<String> {
        let surreal = context.data::<SurrealClient>()?;
        let relate = format!(
            "RELATE {user_id}->match->{user_target_id} SET in_swipe = true, is_super_swipe = true",
            user_id = user_id,
            user_target_id = target_user_id
        );
        surreal.query(relate).await?;

        Ok(String::from("User swiped up"))
    }

    #[graphql(name = "match_users")]
    async fn match_users(
        &self,
        context: &Context<'_>,
        #[graphql(name = "user_id")] user_id: String,
        #[graphql(name = "target_user_id")] target_user_id: String,
    ) -> FieldResult<String> {
        let surreal = context.data::<SurrealClient>()?;
        let query = format!(
            "UPDATE match MERGE {{ out_swipe: true }} WHERE in = {user_id} AND out = {target_user_id}",
            user_id = user_id,
            target_user_id = target_user_id
        );
        surreal.query(query).await?;

        Ok(String::from("Users matched"))
    }

    #[graphql(name = "revert_users_swipe")]
    async fn revert_users_swipe(
        &self,
        context: &Context<'_>,
        #[graphql(name = "user_id")] user_id: String,
        #[graphql(name = "target_user_id")] target_user_id: String,
    ) -> FieldResult<String> {
        let surreal = context.data::<SurrealClient>()?;
        let query = format!(
            "DELETE match WHERE in = {user_id} AND out = {target_user_id}",
            user_id = user_id,
            target_user_id = target_user_id
        );
        surreal.query(query).await?;

        Ok(String::from("Swipe reverted"))
    }
}

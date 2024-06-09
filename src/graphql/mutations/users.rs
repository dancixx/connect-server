use async_graphql::{Context, FieldResult, Object};
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

        Ok(user.ok_or("User not found")?)
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
        let surreal = context.data::<Surreal<Client>>()?;

        // TODO: this is not working
        // let SurrealID(thing) = SurrealID::from(id);
        // let current_location = Point::new(coordinates[0], coordinates[1]);

        // #[derive(Serialize)]
        // struct SurrealPoint {
        //     pub current_location: Point,
        // }

        // let user = surreal
        //     .update::<Option<users::User>>(thing)
        //     .merge(SurrealPoint { current_location })
        //     .await?;

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
    ) -> FieldResult<users::User> {
        let surreal = context.data::<Surreal<Client>>()?;
        let SurrealID(thing) = SurrealID::from(id);
        let user = surreal.delete::<Option<users::User>>(thing).await?;

        Ok(user.ok_or("User not found")?)
    }

    #[graphql(name = "swipe_users_right_by_pk")]
    async fn swipe_users_right_by_pk(
        &self,
        context: &Context<'_>,
        #[graphql(name = "user_id")] user_id: String,
        #[graphql(name = "target_user_id")] target_user_id: String,
    ) -> FieldResult<String> {
        let surreal = context.data::<Surreal<Client>>()?;
        let relate = format!(
            "RELATE {user_id}->users_relations->{user_target_id} SET in_swipe = true",
            user_id = user_id,
            user_target_id = target_user_id
        );
        surreal.query(relate).await?;
        Ok(String::from("User swiped right"))
    }

    #[graphql(name = "swipe_users_left_by_pk")]
    async fn swipe_users_left_by_pk(
        &self,
        context: &Context<'_>,
        #[graphql(name = "user_id")] user_id: String,
        #[graphql(name = "target_user_id")] target_user_id: String,
    ) -> FieldResult<String> {
        let surreal = context.data::<Surreal<Client>>()?;
        let relate = format!(
            "RELATE {user_id}->users_relations->{target_user_id} SET in_swipe = false",
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
        let surreal = context.data::<Surreal<Client>>()?;
        let relate = format!(
            "RELATE {user_id}->users_relations->{user_target_id} SET in_swipe = true, is_super_swipe = true",
            user_id = user_id,
            user_target_id = target_user_id
        );
        surreal.query(relate).await?;
        Ok(String::from("User swiped up"))
    }
}

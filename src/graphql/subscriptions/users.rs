use async_graphql::{Context, Subscription};
use async_stream::stream;
use surrealdb::{Notification, Result};
use tokio_stream::Stream;

use crate::{
    graphql::types::{surreal_id::SurrealID, SurrealClient},
    models::users::User,
};

#[derive(Default)]
pub struct UserSubscriptionRoot;

#[Subscription]
impl UserSubscriptionRoot {
    #[graphql(name = "subscribe_users_by_id")]
    pub async fn subscribe_users_by_id<'a>(
        &self,
        context: &'a Context<'_>,
        id: String,
    ) -> impl Stream<Item = Option<User>> + 'a {
        let surreal = context.data::<SurrealClient>().unwrap();
        let SurrealID(thing) = SurrealID::from(id);
        let initial_data = surreal.select::<Option<User>>(&thing).await.unwrap();
        let stream = surreal.select::<Option<User>>(&thing).live().await.unwrap();

        stream! {
            yield initial_data;

            for await result in stream {
                let result: Result<Notification<User>> = result;

                yield match result {
                    Ok(notification) => Some(notification.data),
                    Err(_) => None,
                }
            }
        }
    }
}

use async_graphql::{Context, Subscription};
use async_stream::stream;
use tokio_stream::Stream;

use crate::{
    graphql::types::{surreal_id::SurrealID, SurrealClient},
    models::users::User,
};

#[derive(Default)]
pub struct UserSubscriptionRoot;

#[Subscription]
impl UserSubscriptionRoot {
    #[graphql(name = "select_users_by_id")]
    pub async fn select_users_by_id<'a>(
        &self,
        context: &'a Context<'_>,
        id: String,
    ) -> impl Stream<Item = Option<User>> + 'a {
        let surreal = context.data::<SurrealClient>().unwrap();
        let SurrealID(thing) = SurrealID::from(id);
        let query = surreal.select(&thing).await.unwrap();
        let stream = surreal.select(&thing).live().await.unwrap();

        stream! {
            yield query;

            for await result in stream {
                yield match result {
                    Ok(notification) => Some(notification.data),
                    Err(_) => None,
                }
            }
        }
    }
}

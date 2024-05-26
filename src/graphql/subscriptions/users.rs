use std::{sync::Arc, time::Duration};

use async_graphql::{Context, Subscription};
use surrealdb::{engine::remote::ws::Client, Surreal};
use tokio::sync::RwLock;
use tokio_stream::{Stream, StreamExt};

use crate::{graphql::types::surreal_id::SurrealID, models::users::User};

#[derive(Default)]
pub struct UsersSubscriptionRoot;

#[Subscription]
impl UsersSubscriptionRoot {
    #[graphql(name = "subscribe_users_by_id")]
    pub async fn subscribe_users_by_id<'a>(
        &self,
        context: &Context<'_>,
        id: String,
    ) -> impl Stream<Item = Option<User>> + 'a {
        let surreal = context.data::<Surreal<Client>>().unwrap();
        let surreal = Arc::new(RwLock::new(surreal.clone()));
        let SurrealID(thing) = SurrealID::from(id);

        tokio_stream::wrappers::IntervalStream::new(tokio::time::interval(Duration::from_secs(1)))
            .then(move |_| {
                let surreal = surreal.clone();
                let id = thing.id.clone();

                async move {
                    let surreal = surreal.read().await;
                    match surreal.select::<Option<User>>(("users", id.clone())).await {
                        Ok(Some(user)) => Some(user),
                        _ => None,
                    }
                }
            })
    }
}

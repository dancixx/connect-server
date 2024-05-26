use std::{
    mem,
    sync::{Arc, Mutex},
    time::Duration,
};

use async_graphql::{Context, Subscription};
use surrealdb::{engine::remote::ws::Client, Surreal};
use tokio::{runtime::Handle, sync::RwLock};
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
    ) -> impl Stream<Item = User> + 'a {
        let surreal = context.data::<Surreal<Client>>().unwrap();
        let surreal = Arc::new(RwLock::new(surreal.clone()));
        let SurrealID(thing) = SurrealID::from(id);

        tokio_stream::wrappers::IntervalStream::new(tokio::time::interval(Duration::from_secs(1)))
            .map(move |_| {
                let id = thing.id.clone();
                let user = Arc::new(Mutex::<Option<User>>::new(None));
                let surreal = surreal.clone();

                tokio::task::block_in_place({
                    let user = user.clone();
                    let surreal = surreal.clone();

                    move || {
                        let user = user.clone();
                        let surreal = surreal.clone();

                        Handle::current().block_on(async move {
                            let surreal = surreal.read().await;
                            let query =
                                surreal.select::<Option<User>>(("users", id)).await.unwrap();

                            let mut user = user.lock().unwrap();
                            *user = query;
                        });
                    }
                });

                let mut user = user.lock().unwrap();
                mem::take(&mut *user).unwrap()
            })
    }
}

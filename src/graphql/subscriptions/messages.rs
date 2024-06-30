use std::{collections::VecDeque, sync::Arc};

use async_graphql::{Context, Subscription};
use async_stream::stream;
use surrealdb::{Action, Notification, Result};
use tokio::sync::Mutex;
use tokio_stream::Stream;

use crate::{graphql::types::SurrealClient, models::messages};

#[derive(Default)]
pub struct MessageSubscriptionRoot;

#[Subscription]
impl MessageSubscriptionRoot {
    #[graphql(name = "select_chat_by_users")]
    pub async fn select_chat_by_users<'a>(
        &self,
        context: &'a Context<'_>,
        #[graphql(name = "match_id")] match_id: String,
    ) -> impl Stream<Item = Option<VecDeque<messages::Message>>> + 'a {
        let surreal = context.data::<SurrealClient>().unwrap();
        // TODO: add pagination
        let mut query_result = surreal
            .query(format!(
                "SELECT *, out.* as user FROM message WHERE in = {0} ORDER BY created_at DESC;",
                match_id
            ))
            .await
            .unwrap();
        let messages = query_result
            .take::<Vec<messages::Message>>(0)
            .unwrap_or_default();

        let mut stream_result = surreal
            .query(format!(
                "LIVE SELECT *, out.* as user FROM message WHERE in = {0}",
                match_id
            ))
            .await
            .unwrap();
        let stream_messages = stream_result
            .stream::<Notification<messages::Message>>(0)
            .unwrap();
        let messages = Arc::new(Mutex::new(VecDeque::from(messages)));

        stream! {
            yield Some(messages.lock().await.clone());

            for await result in stream_messages {
                let result: Result<Notification<messages::Message>> = result;

                yield match result {
                    Ok(notification) => {
                        let message = notification.data;

                        match notification.action {
                            Action::Create => {
                                messages.lock().await.push_front(message.clone());
                            },
                            Action::Update => {
                                let index = messages.lock().await.iter().position(|m| m.id == message.id);
                                if let Some(index) = index {
                                    messages.lock().await[index] = message.clone();
                                }
                            },
                            Action::Delete => {
                                let index = messages.lock().await.iter().position(|m| m.id == message.id);
                                if index.is_some() {
                                    messages.lock().await.remove(index.unwrap());
                                }
                            },
                            _ => {},
                        }

                        Some(messages.lock().await.clone())
                    },
                    Err(_) => None,
                }
            }
        }
    }
}

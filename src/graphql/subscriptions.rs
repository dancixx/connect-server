use async_graphql::*;

pub mod users;

#[derive(MergedSubscription, Default)]
pub struct SubscriptionRoot(self::users::UsersSubscriptionRoot);

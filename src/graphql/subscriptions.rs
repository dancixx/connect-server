use async_graphql::*;

pub mod messages;
pub mod users;

#[derive(MergedSubscription, Default)]
pub struct SubscriptionRoot(
    self::messages::MessageSubscriptionRoot,
    self::users::UserSubscriptionRoot,
);

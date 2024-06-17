use async_graphql::MergedObject;

pub mod ai;
pub mod chats;
pub mod goals;
pub mod interests;
pub mod users;

#[derive(MergedObject, Default)]
pub struct QueryRoot(
    self::ai::AiQueryRoot,
    self::chats::ChatsQueryRoot,
    self::goals::GoalsQueryRoot,
    self::interests::InterestsQueryRoot,
    self::users::UsersQueryRoot,
);

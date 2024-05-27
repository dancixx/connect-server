use async_graphql::MergedObject;

pub mod ai;
pub mod users;

#[derive(MergedObject, Default)]
pub struct QueryRoot(self::ai::AiQueryRoot, self::users::UsersQueryRoot);

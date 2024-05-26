use async_graphql::MergedObject;

pub mod users;

#[derive(MergedObject, Default)]
pub struct MutationRoot(self::users::UsersMutationRoot);

use async_graphql::MergedObject;

pub mod goals;
pub mod interests;
pub mod user_images;
pub mod users;

#[derive(MergedObject, Default)]
pub struct MutationRoot(
    self::goals::GoalsMutationRoot,
    self::interests::InterestsMutationRoot,
    self::users::UsersMutationRoot,
    self::user_images::UserImagesMutationRoot,
);

use async_graphql::MergedObject;

pub mod messages;
pub mod user_images;
pub mod users;

#[derive(MergedObject, Default)]
pub struct MutationRoot(
    self::messages::MessageMutationRoot,
    self::users::UserMutationRoot,
    self::user_images::UserImageMutationRoot,
);

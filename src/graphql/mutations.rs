use async_graphql::MergedObject;

pub mod user_images;
pub mod users;

#[derive(MergedObject, Default)]
pub struct MutationRoot(
    self::users::UsersMutationRoot,
    self::user_images::UserImagesMutationRoot,
);

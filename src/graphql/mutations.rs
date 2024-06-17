use async_graphql::MergedObject;

pub mod chats;
pub mod user_images;
pub mod users;

#[derive(MergedObject, Default)]
pub struct MutationRoot(
    self::chats::ChatsMutationRoot,
    self::users::UsersMutationRoot,
    self::user_images::UserImagesMutationRoot,
);

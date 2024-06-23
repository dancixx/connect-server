use async_graphql::MergedObject;

pub mod ai;
pub mod chats;
pub mod i18n;
pub mod users;

#[derive(MergedObject, Default)]
pub struct QueryRoot(
    self::ai::AiQueryRoot,
    self::chats::ChatsQueryRoot,
    self::i18n::I18nQueryRoot,
    self::users::UsersQueryRoot,
);

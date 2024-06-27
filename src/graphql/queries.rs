use async_graphql::MergedObject;

pub mod ai;
pub mod i18n;
pub mod messages;
pub mod users;

#[derive(MergedObject, Default)]
pub struct QueryRoot(
    self::ai::AiQueryRoot,
    self::messages::MessageQueryRoot,
    self::i18n::I18nQueryRoot,
    self::users::UserQueryRoot,
);

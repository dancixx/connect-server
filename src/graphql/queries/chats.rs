use async_graphql::Object;

#[derive(Default)]
pub struct ChatsQueryRoot;

#[Object]
impl ChatsQueryRoot {
    async fn select_chats(&self) -> Vec<String> {
        vec!["chat1".to_string(), "chat2".to_string()]
    }
}

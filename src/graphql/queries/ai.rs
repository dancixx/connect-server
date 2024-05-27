use async_graphql::{FieldResult, Object};

#[derive(Default)]
pub struct AiQueryRoot;

#[Object]
impl AiQueryRoot {
    #[graphql(name = "ask_ai")]
    pub async fn ask_ai(&self) -> FieldResult<String> {
        unimplemented!()
    }
}

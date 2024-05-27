use async_graphql::*;
use surrealdb::sql::Thing;

#[derive(Default)]
pub struct UserImagesMutationRoot;

#[Object]
impl UserImagesMutationRoot {
    #[graphql(name = "insert_user_images")]
    async fn insert_user_images(
        &self,
        context: &Context<'_>,
        user_id: String,
    ) -> FieldResult<String> {
        unimplemented!()
    }
}

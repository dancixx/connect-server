use async_graphql::*;

#[derive(Default)]
pub struct UserImageMutationRoot;

#[Object]
impl UserImageMutationRoot {
    #[graphql(name = "insert_user_images")]
    async fn insert_user_images(
        &self,
        _context: &Context<'_>,
        _user_id: String,
    ) -> FieldResult<String> {
        unimplemented!()
    }
}

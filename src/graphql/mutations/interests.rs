use async_graphql::{Context, FieldResult, Object};
use surrealdb::{engine::remote::ws::Client, Surreal};

use crate::{graphql::types::surreal_id::SurrealID, models::edge::Edge};

#[derive(Default)]
pub struct InterestsMutationRoot;

#[Object]
impl InterestsMutationRoot {
    #[graphql(name = "insert_interests_many")]
    pub async fn insert_interests_many(
        &self,
        context: &Context<'_>,
        #[graphql(name = "user_id")] user_id: String,
        #[graphql(name = "ids")] ids: Vec<String>,
    ) -> FieldResult<String> {
        let surreal = context.data::<Surreal<Client>>()?;
        surreal
            .insert::<Vec<Edge>>("interests_relations")
            .content(
                ids.iter()
                    .map(|interest_id| Edge {
                        r#in: user_id.clone(),
                        out: interest_id.clone(),
                    })
                    .collect::<Vec<Edge>>(),
            )
            .await
            .unwrap();
        Ok(String::from("Inserted"))
    }

    #[graphql(name = "insert_interests_one")]
    pub async fn insert_interests_one(
        &self,
        context: &Context<'_>,
        #[graphql(name = "user_id")] user_id: String,
        #[graphql(name = "interest_id")] interest_id: String,
    ) -> FieldResult<String> {
        let surreal = context.data::<Surreal<Client>>()?;
        surreal
            .insert::<Vec<Edge>>("interests_relations")
            .content(vec![Edge {
                r#in: user_id,
                out: interest_id,
            }])
            .await
            .unwrap();
        Ok(String::from("Inserted"))
    }

    #[graphql(name = "delete_interests_one")]
    pub async fn delete_interests_one(
        &self,
        context: &Context<'_>,
        id: String,
    ) -> FieldResult<String> {
        let surreal = context.data::<Surreal<Client>>()?;
        let SurrealID(thing) = SurrealID::from(id);
        surreal.delete::<Option<Edge>>(thing).await?;
        Ok(String::from("Deleted"))
    }
}

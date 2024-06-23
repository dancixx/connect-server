use async_graphql::{Context, FieldResult, Object};

use crate::{
    graphql::types::{I18nTables, SurrealClient},
    models::i18n::I18n,
};

#[derive(Default)]
pub struct I18nQueryRoot;

#[Object]
impl I18nQueryRoot {
    #[graphql(name = "select_alcohols")]
    async fn select_alcohols(
        &self,
        context: &Context<'_>,
        #[graphql(name = "order_key", default = "en")] order_key: String,
    ) -> FieldResult<Vec<I18n>> {
        let surreal = context.data::<SurrealClient>()?;
        let query = format!(
            "SELECT * FROM {} ORDER BY {}",
            I18nTables::i18n_alcohol,
            order_key
        );
        let query = surreal.query(query).await;

        if let Err(e) = query {
            tracing::error!("Error: {:?}", e);
            return Err(e.into());
        }

        let alcohols = query?.take::<Vec<I18n>>(0)?;
        Ok(alcohols)
    }

    #[graphql(name = "select_educations")]
    async fn select_educations(
        &self,
        context: &Context<'_>,
        #[graphql(name = "order_key", default = "en")] order_key: String,
    ) -> FieldResult<Vec<I18n>> {
        let surreal = context.data::<SurrealClient>()?;
        let query = format!(
            "SELECT * FROM {} ORDER BY {}",
            I18nTables::i18n_education,
            order_key
        );
        let query = surreal.query(query).await;

        if let Err(e) = query {
            tracing::error!("Error: {:?}", e);
            return Err(e.into());
        }

        let eductions = query?.take::<Vec<I18n>>(0)?;
        Ok(eductions)
    }

    #[graphql(name = "select_goals")]
    async fn select_goals(
        &self,
        context: &Context<'_>,
        #[graphql(name = "order_key", default = "en")] order_key: String,
    ) -> FieldResult<Vec<I18n>> {
        let surreal = context.data::<SurrealClient>()?;
        let query = format!(
            "SELECT * FROM {} ORDER BY {}",
            I18nTables::i18n_goal,
            order_key
        );
        let query = surreal.query(query).await;

        if let Err(e) = query {
            tracing::error!("Error: {:?}", e);
            return Err(e.into());
        }

        let goals = query?.take::<Vec<I18n>>(0)?;
        Ok(goals)
    }

    #[graphql(name = "select_interests")]
    async fn select_interests(
        &self,
        context: &Context<'_>,
        #[graphql(name = "order_key", default = "en")] order_key: String,
    ) -> FieldResult<Vec<I18n>> {
        let surreal = context.data::<SurrealClient>()?;
        let query = format!(
            "SELECT * FROM {} ORDER BY {}",
            I18nTables::i18n_interest,
            order_key
        );
        let query = surreal.query(query).await;

        if let Err(e) = query {
            tracing::error!("Error: {:?}", e);
            return Err(e.into());
        }

        let interests = query?.take::<Vec<I18n>>(0)?;
        Ok(interests)
    }

    #[graphql(name = "select_languages")]
    async fn select_languages(
        &self,
        context: &Context<'_>,
        #[graphql(name = "order_key", default = "en")] order_key: String,
    ) -> FieldResult<Vec<I18n>> {
        let surreal = context.data::<SurrealClient>()?;
        let query = format!(
            "SELECT * FROM {} ORDER BY {}",
            I18nTables::i18n_language,
            order_key
        );
        let query = surreal.query(query).await;

        if let Err(e) = query {
            tracing::error!("Error: {:?}", e);
            return Err(e.into());
        }

        let languages = query?.take::<Vec<I18n>>(0)?;
        Ok(languages)
    }

    #[graphql(name = "select_politicals")]
    async fn select_politicals(
        &self,
        context: &Context<'_>,
        #[graphql(name = "order_key", default = "en")] order_key: String,
    ) -> FieldResult<Vec<I18n>> {
        let surreal = context.data::<SurrealClient>()?;
        let query = format!(
            "SELECT * FROM {} ORDER BY {}",
            I18nTables::i18n_political,
            order_key
        );
        let query = surreal.query(query).await;

        if let Err(e) = query {
            tracing::error!("Error: {:?}", e);
            return Err(e.into());
        }

        let politicals = query?.take::<Vec<I18n>>(0)?;
        Ok(politicals)
    }

    #[graphql(name = "select_religions")]
    async fn select_religions(
        &self,
        context: &Context<'_>,
        #[graphql(name = "order_key", default = "en")] order_key: String,
    ) -> FieldResult<Vec<I18n>> {
        let surreal = context.data::<SurrealClient>()?;
        let query = format!(
            "SELECT * FROM {} ORDER BY {}",
            I18nTables::i18n_religion,
            order_key
        );
        let query = surreal.query(query).await;

        if let Err(e) = query {
            tracing::error!("Error: {:?}", e);
            return Err(e.into());
        }

        let religions = query?.take::<Vec<I18n>>(0)?;
        Ok(religions)
    }

    #[graphql(name = "select_zodiacs")]
    async fn select_zodiacs(
        &self,
        context: &Context<'_>,
        #[graphql(name = "order_key", default = "en")] order_key: String,
    ) -> FieldResult<Vec<I18n>> {
        let surreal = context.data::<SurrealClient>()?;
        let query = format!(
            "SELECT * FROM {} ORDER BY {}",
            I18nTables::i18n_zodiac,
            order_key
        );
        let query = surreal.query(query).await;

        if let Err(e) = query {
            tracing::error!("Error: {:?}", e);
            return Err(e.into());
        }

        let zodiacs = query?.take::<Vec<I18n>>(0)?;
        Ok(zodiacs)
    }
}

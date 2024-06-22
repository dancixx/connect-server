use std::env;

use anyhow::Result;
use surrealdb::{engine::remote::ws::Ws, opt::auth::Root, Surreal};
use surrealdb_migrations::MigrationRunner;

use crate::graphql::types::SurrealClient;

pub async fn init() -> Result<SurrealClient> {
    let surreal = Surreal::new::<Ws>("127.0.0.1:8000").await?;
    surreal
        .signin(Root {
            username: &env::var("SURREAL_ROOT_USER")?,
            password: &env::var("SURREAL_ROOT_PASS")?,
        })
        .await?;
    tracing::info!("Signed in to surreal");
    surreal
        .use_ns(&env::var("SURREAL_NS")?)
        .use_db(&env::var("SURREAL_DB")?)
        .await?;

    Ok(surreal)
}

pub async fn run_migrations(surreal: &SurrealClient) -> Result<()> {
    // Apply new migrations
    MigrationRunner::new(surreal).up().await.unwrap();

    // List applied migrations
    let applied_migrations = MigrationRunner::new(surreal).list().await.unwrap();
    tracing::info!("Applied migrations: {:?}", applied_migrations);

    Ok(())
}

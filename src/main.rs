mod graphql;
mod models;

// mod marcos;
// mod schema;

use std::{env, ops::DerefMut};

use anyhow::Result;
use async_graphql::{EmptySubscription, Schema};
use axum::{http::Method, routing::get, Router};
use deadpool_postgres::{Manager, Pool};
use firebase_auth::FirebaseAuth;
use graphql::{MutationRoot, QueryRoot};
use tokio::net::TcpListener;
use tokio_postgres::{config::Config, NoTls};
use tower_http::cors::{AllowOrigin, CorsLayer};

// use crate::graphql::QueryRoot;

refinery::embed_migrations!("migrations");

#[derive(Clone)]
pub struct AppState {
    #[allow(dead_code)]
    firebase_auth: FirebaseAuth,
    schema: Schema<QueryRoot, MutationRoot, EmptySubscription>,
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().expect("Failed to load .env file");
    let mut cfg = Config::new();
    cfg.host(&env::var("DB_HOST")?);
    cfg.port(env::var("DB_PORT")?.parse::<u16>()?);
    cfg.user(&env::var("DB_USER")?);
    cfg.password(&env::var("DB_PASSWORD")?);
    let mgr = Manager::new(cfg, NoTls);
    let pool = Pool::builder(mgr).build()?;
    // Run the embedded migrations
    let mut postgres_client = pool.get().await?;
    let postgres_client = postgres_client.deref_mut();
    let postgres_client = postgres_client.deref_mut();
    let report = migrations::runner().run_async(postgres_client).await?;
    for migration in report.applied_migrations() {
        println!("Migration Applied: {}", migration);
    }
    // Initialize Redis
    let _redis = redis::Client::open(env::var("REDIS_URL")?)?;

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::WARN)
        .init();

    println!("GraphiQL IDE: http://localhost:8080");
    let schema = Schema::build(
        QueryRoot::default(),
        MutationRoot::default(),
        EmptySubscription,
    )
    .data(pool)
    .data(_redis)
    .finish();

    let firebase_auth = FirebaseAuth::new(&std::env::var("FIREBASE_PROJECT_ID")?).await;
    let app = Router::new()
        .route("/", get(graphql::playground).post(graphql::handler))
        .with_state(AppState {
            firebase_auth,
            schema,
        })
        .layer(
            CorsLayer::new()
                .allow_origin(AllowOrigin::predicate(|_, _| true))
                .allow_methods([Method::GET, Method::POST]),
        );

    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    tracing::debug!("listening on {}", listener.local_addr()?);
    axum::serve(listener, app).await?;

    Ok(())
}

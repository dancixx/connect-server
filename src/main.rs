mod graphql;
mod models;

// mod marcos;
// mod schema;

use std::env;

use anyhow::Result;
use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use axum::{http::Method, routing::get, Router};
use firebase_auth::FirebaseAuth;
use graphql::{MutationRoot, QueryRoot};
use surrealdb::{engine::remote::ws::Ws, opt::auth::Root, Surreal};
use tokio::net::TcpListener;
use tower_http::cors::{AllowOrigin, CorsLayer};

#[derive(Clone)]
pub struct AppState {
    #[allow(dead_code)]
    firebase_auth: FirebaseAuth,
    schema: Schema<QueryRoot, EmptyMutation, EmptySubscription>,
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().expect("Failed to load .env file");

    // Initialize Redis
    let _redis = redis::Client::open(env::var("REDIS_URL")?)?;

    // Initialize surrealdb
    let surreal = Surreal::new::<Ws>("127.0.0.1:8000").await?;
    // Sign in as root user
    surreal
        .signin(Root {
            username: "root",
            password: "root",
        })
        .await?;
    // Create a new namespace
    surreal.use_ns("connect").use_db("connect").await?;

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::WARN)
        .init();

    println!("GraphiQL IDE: http://localhost:8080");
    let schema = Schema::build(QueryRoot::default(), EmptyMutation, EmptySubscription)
        .data(_redis)
        .data(surreal)
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

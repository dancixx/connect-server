mod enums;
mod graphql;
mod models;
mod surreal;

use std::env;

use anyhow::Result;
use async_graphql::{extensions::Logger, Schema};
use async_openai::Client;
use axum::{http::Method, routing::get, Router};
use firebase_auth::FirebaseAuth;
use graphql::{mutations::MutationRoot, queries::QueryRoot, subscriptions::SubscriptionRoot};
use tokio::net::TcpListener;
use tower_http::{
    cors::{AllowOrigin, CorsLayer},
    trace::TraceLayer,
};

pub type GraphqlSchema = Schema<QueryRoot, MutationRoot, SubscriptionRoot>;

#[derive(Clone)]
pub struct AppState {
    firebase_auth: FirebaseAuth,
    schema: GraphqlSchema,
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().expect("Failed to load .env file");
    tracing_subscriber::fmt()
        .with_file(true)
        .with_line_number(true)
        .with_level(true)
        .with_max_level(tracing::Level::INFO)
        .init();

    let _redis = redis::Client::open(env::var("REDIS_URL")?)?;
    let openai = Client::new();
    let surreal = surreal::init().await?;

    //surreal::run_migrations(&surreal).await?;

    tracing::info!("GraphiQL IDE: http://localhost:8080");
    let firebase_auth = FirebaseAuth::new(&std::env::var("FIREBASE_PROJECT_ID")?).await;
    let schema = Schema::build(
        QueryRoot::default(),
        MutationRoot::default(),
        SubscriptionRoot::default(),
    )
    .data(_redis)
    .data(surreal)
    .data(openai)
    .data(firebase_auth.clone())
    .extension(Logger)
    .finish();

    let app_state = AppState {
        firebase_auth,
        schema,
    };
    let app = Router::new()
        .route("/", get(graphql::playground).post(graphql::http_handler))
        .route("/ws", get(graphql::ws_handler))
        .layer(
            CorsLayer::new()
                .allow_origin(AllowOrigin::predicate(|_, _| true))
                .allow_methods([Method::GET, Method::POST]),
        )
        .layer(TraceLayer::new_for_http())
        .with_state(app_state);

    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    tracing::debug!("listening on {}", listener.local_addr()?);
    axum::serve(listener, app).await?;

    Ok(())
}

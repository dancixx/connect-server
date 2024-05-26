mod graphql;
mod models;

use std::env;

use anyhow::Result;
use async_graphql::{extensions::Logger, Schema};
use async_graphql_axum::{GraphQL, GraphQLSubscription};
use axum::{http::Method, middleware, routing::get, Router};
use firebase_auth::FirebaseAuth;
use graphql::{mutations::MutationRoot, queries::QueryRoot, subscriptions::SubscriptionRoot};
use surrealdb::{engine::remote::ws::Ws, opt::auth::Root, Surreal};
use surrealdb_migrations::MigrationRunner;
use tokio::net::TcpListener;
use tower_http::{
    cors::{AllowOrigin, CorsLayer},
    trace::TraceLayer,
};

#[derive(Clone)]
pub struct AppState {
    #[allow(dead_code)]
    firebase_auth: FirebaseAuth,
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

    if env::var("CARGO_WATCH").is_ok() {
        tracing::info!("Cargo watch enabled, skipping migration list");
    } else {
        // Apply new migrations
        MigrationRunner::new(&surreal).up().await.unwrap();

        // List applied migrations
        let applied_migrations = MigrationRunner::new(&surreal).list().await.unwrap();
        tracing::info!("Applied migrations: {:?}", applied_migrations);
    }

    tracing::info!("GraphiQL IDE: http://localhost:8080");
    let schema = Schema::build(
        QueryRoot::default(),
        MutationRoot::default(),
        SubscriptionRoot::default(),
    )
    .data(_redis)
    .data(surreal)
    .extension(Logger)
    .finish();

    let firebase_auth = FirebaseAuth::new(&std::env::var("FIREBASE_PROJECT_ID")?).await;
    let app_state = AppState { firebase_auth };
    let app = Router::new()
        .route(
            "/",
            get(graphql::graphiql).post_service(GraphQL::new(schema.clone())),
        )
        .route_service("/ws", GraphQLSubscription::new(schema.clone()))
        .layer(
            CorsLayer::new()
                .allow_origin(AllowOrigin::predicate(|_, _| true))
                .allow_methods([Method::GET, Method::POST]),
        )
        .layer(TraceLayer::new_for_http())
        // .layer(middleware::from_fn_with_state(
        //     app_state.clone(),
        //     graphql::auth_handler,
        // ))
        .with_state(app_state);

    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    tracing::debug!("listening on {}", listener.local_addr()?);
    axum::serve(listener, app).await?;

    Ok(())
}

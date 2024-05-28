pub mod mutations;
pub mod queries;
pub mod subscriptions;
pub mod types;

use std::env;

use async_graphql::http::GraphiQLSource;
use axum::{
    extract::{Request, State},
    http::{header::AUTHORIZATION, HeaderMap},
    middleware::Next,
    response::{Html, IntoResponse, Response},
};
use firebase_auth::FirebaseUser;

use crate::AppState;

pub async fn graphiql() -> impl IntoResponse {
    Html(
        GraphiQLSource::build()
            .endpoint("/")
            .subscription_endpoint("/ws")
            .finish(),
    )
}

pub async fn auth_handler(
    State(AppState { firebase_auth, .. }): State<AppState>,
    headers: HeaderMap,
    req: Request,
    next: Next,
) -> Response {
    let admin_secret = headers.get("x-connect-admin-secret");
    let _admin_secret = env::var("ADMIN_SECRET").unwrap();

    if let Some(admin_secret) = admin_secret {
        if admin_secret == &_admin_secret {
            return next.run(req).await;
        }
    }

    let auth = headers.get(AUTHORIZATION);
    match auth {
        Some(auth) => {
            let auth_header = auth.to_str().unwrap();
            if auth_header.is_empty() {
                tracing::info!("Token is empty");
                return Response::new("Token is empty".into());
            }

            let prefix_len = "Bearer ".len();
            if auth_header.len() <= prefix_len {
                tracing::info!("Token is empty");
                return Response::new("Token is empty".into());
            }

            let token = auth_header[prefix_len..].to_string();

            match firebase_auth.verify::<FirebaseUser>(&token) {
                Ok(_) => next.run(req).await,
                Err(_) => {
                    tracing::info!("Invalid token");
                    Response::new("Invalid token".into())
                }
            }
        }
        None => {
            tracing::info!("No Authorization header");
            Response::new("No Authorization header".into())
        }
    }
}

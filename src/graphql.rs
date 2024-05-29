pub mod mutations;
pub mod queries;
pub mod subscriptions;
pub mod types;

use std::env;

use async_graphql::{
    http::{GraphiQLSource, ALL_WEBSOCKET_PROTOCOLS},
    Data, Response,
};
use async_graphql_axum::{GraphQLProtocol, GraphQLRequest, GraphQLResponse, GraphQLWebSocket};
use axum::{
    extract::{State, WebSocketUpgrade},
    response::{Html, IntoResponse, Response as AxumResponse},
};
use axum_auth::AuthBearer;
use firebase_auth::FirebaseUser;
use serde::Deserialize;

use crate::AppState;

pub async fn playground() -> impl IntoResponse {
    Html(
        GraphiQLSource::build()
            .endpoint("/")
            .subscription_endpoint("/ws")
            .finish(),
    )
}

pub async fn http_handler(
    State(AppState {
        firebase_auth,
        schema,
    }): State<AppState>,
    AuthBearer(token): AuthBearer,
    req: GraphQLRequest,
) -> GraphQLResponse {
    let admin_secret = env::var("ADMIN_SECRET").unwrap();

    if token == admin_secret {
        return schema.execute(req.into_inner()).await.into();
    }

    if token.is_empty() {
        tracing::error!("Token is empty");
        return GraphQLResponse::from(Response::new("Token is empty"));
    }

    match firebase_auth.verify::<FirebaseUser>(&token) {
        Ok(_) => schema.execute(req.into_inner()).await.into(),
        Err(_) => GraphQLResponse::from(Response::new("Invalid token")),
    }
}

#[derive(Deserialize, Debug)]
struct WebSocketAuthPayload {
    token: String,
}

pub async fn ws_handler(
    State(AppState {
        firebase_auth,
        schema,
    }): State<AppState>,
    protocol: GraphQLProtocol,
    websocket: WebSocketUpgrade,
) -> AxumResponse {
    websocket
        .protocols(ALL_WEBSOCKET_PROTOCOLS)
        .on_upgrade(move |stream| {
            GraphQLWebSocket::new(stream, schema.clone(), protocol)
                .on_connection_init(move |value: serde_json::Value| {
                    let firebase_auth = firebase_auth.clone();
                    async move {
                        if let Ok(payload) = serde_json::from_value::<WebSocketAuthPayload>(value) {
                            let token = payload.token[7..].as_ref();

                            if firebase_auth.verify::<FirebaseUser>(token).is_ok() {
                                tracing::info!("User connected");
                                Ok(Data::default())
                            } else {
                                tracing::error!("Invalid token");
                                Err("Invalid token".into())
                            }
                        } else {
                            tracing::error!("Token is required");
                            Err("Token is required".into())
                        }
                    }
                })
                .serve()
        })
}

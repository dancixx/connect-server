pub mod mutations;
pub mod queries;
pub mod types;

// use std::env;

use std::env;

// use anyhow::Result;
use async_graphql::{http::GraphiQLSource, MergedObject, Response};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    extract::State,
    http::{header::AUTHORIZATION, HeaderMap},
    response::{Html, IntoResponse},
};
use firebase_auth::FirebaseUser;
// use surrealdb::{engine::remote::ws::Ws, opt::auth::Scope, sql::Thing, Surreal};

use crate::AppState;

use self::types::UserID;

#[derive(MergedObject, Default)]
pub struct QueryRoot(queries::users::UsersQueryRoot);

#[derive(MergedObject, Default)]
pub struct MutationRoot(mutations::users::UsersMutationRoot);

pub async fn playground() -> impl IntoResponse {
    Html(GraphiQLSource::build().endpoint("/").finish())
}

pub async fn handler(
    State(AppState {
        firebase_auth,
        schema,
    }): State<AppState>,
    headers: HeaderMap,
    req: GraphQLRequest,
) -> GraphQLResponse {
    let admin_secret = headers.get("x-connect-admin-secret");
    let _admin_secret = env::var("ADMIN_SECRET").unwrap();

    if let Some(admin_secret) = admin_secret {
        if admin_secret == &_admin_secret {
            return schema.execute(req.into_inner()).await.into();
        }
    }

    let auth = headers.get(AUTHORIZATION);
    match auth {
        Some(auth) => {
            let auth_header = auth.to_str().unwrap();
            if auth_header.is_empty() {
                tracing::info!("Token is empty");
                return GraphQLResponse::from(Response::new("Token is empty"));
            }

            let prefix_len = "Bearer ".len();
            if auth_header.len() <= prefix_len {
                tracing::info!("Token is empty");
                return GraphQLResponse::from(Response::new("Token is empty"));
            }

            let token = auth_header[prefix_len..].to_string();

            match firebase_auth.verify::<FirebaseUser>(&token) {
                Ok(firebase_user) => {
                    // let surreal = Surreal::new::<Ws>("127.0.0.1:8000").await.unwrap();
                    // surreal
                    //     .signin(Scope {
                    //         namespace: &env::var("SURREAL_NS").unwrap(),
                    //         database: &env::var("SURREAL_DB").unwrap(),
                    //         scope: "account",
                    //         params: SurrealID(Thing {
                    //             tb: String::from("users"),
                    //             id: firebase_user.user_id.into(),
                    //         }),
                    //     })
                    //     .await
                    //     .unwrap();
                    schema
                        .execute(req.into_inner().data(UserID(firebase_user.user_id)))
                        .await
                        .into()
                }
                Err(_) => {
                    tracing::info!("Invalid token");
                    GraphQLResponse::from(Response::new("Invalid token"))
                }
            }
        }
        None => {
            tracing::info!("No Authorization header");
            GraphQLResponse::from(Response::new("No Authorization header"))
        }
    }
}

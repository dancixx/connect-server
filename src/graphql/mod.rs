pub mod queries;

use async_graphql::{http::GraphiQLSource, MergedObject, Response};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    extract::State,
    http::{header::AUTHORIZATION, HeaderMap},
    response::{Html, IntoResponse},
};

use firebase_auth::FirebaseUser;

use crate::AppState;

use self::queries::users::Users;

#[derive(MergedObject, Default)]
pub struct QueryRoot(Users);

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
    let auth = headers.get(AUTHORIZATION);
    return schema.execute(req.into_inner()).await.into();
    match auth {
        Some(auth) => {
            let auth_header = auth.to_str().unwrap();
            if auth_header.is_empty() {
                return GraphQLResponse::from(Response::new("Token is empty"));
            }

            let prefix_len = "Bearer ".len();
            if auth_header.len() <= prefix_len {
                return GraphQLResponse::from(Response::new("Token is empty"));
            }

            let token = auth_header[prefix_len..].to_string();
            match firebase_auth.verify::<FirebaseUser>(&token) {
                Ok(_) => schema.execute(req.into_inner()).await.into(),
                Err(_) => return GraphQLResponse::from(Response::new("Invalid token")),
            }
        }
        None => {
            return GraphQLResponse::from(Response::new("No Authorization header"));
        }
    }
}

use crate::templates::IndexTemplate;
use askama_axum::IntoResponse;
use axum::body::Body;
use axum::extract::Query;
use axum::routing::get;
use axum::Router;
use http::StatusCode;
use tower_http::services::ServeDir;
use serde::Deserialize;

pub struct BaseRouter;

impl BaseRouter {
    pub fn new_router() -> Router {
        Router::new()
            .nest_service("/dist", ServeDir::new("dist"))
            .route(
                "/ping",
                get(|| async { (StatusCode::OK, Body::from("pong")) }),
            )
            .route("/", get(get_index))
    }
}

#[derive(Deserialize)]
struct IndexParams {
    size: Option<usize>,
}

async fn get_index(params: Query<IndexParams>) -> impl IntoResponse {
    IndexTemplate { size: params.size.unwrap_or(4) }.into_response()
}

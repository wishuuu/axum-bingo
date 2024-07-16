use crate::templates::IndexTemplate;
use askama_axum::IntoResponse;
use axum::body::Body;
use axum::routing::get;
use axum::Router;
use http::StatusCode;
use tower_http::services::ServeDir;

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

async fn get_index() -> impl IntoResponse {
    IndexTemplate {}.into_response()
}

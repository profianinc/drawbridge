use axum::handler::Handler;
use axum::http::StatusCode;
use axum::Router;

pub fn new() -> Router {
    Router::new().fallback(
        (|| async { (StatusCode::NOT_IMPLEMENTED, "Tree handling not implemented") })
            .into_service(),
    )
}

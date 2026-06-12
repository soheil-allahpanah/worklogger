mod worklogs;

use axum::{
    routing::{delete, get, post},
    Router,
};
use tower_http::trace::TraceLayer;

use crate::state::AppState;

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/worklogs", post(worklogs::create).get(worklogs::filter_query))
        .route("/worklogs/filter", post(worklogs::filter))
        .route("/worklogs/export", get(worklogs::export_query).post(worklogs::export))
        .route("/worklogs/{id}", delete(worklogs::delete))
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}

async fn health() -> &'static str {
    "ok"
}

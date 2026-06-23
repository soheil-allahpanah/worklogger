use axum::{
    routing::{delete, get, post},
    Router,
};
use tower_http::trace::TraceLayer;

use crate::routes::controllers;
use crate::state::AppState;

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health))
        .route(
            "/worklogs",
            post(controllers::create).get(controllers::filter_query),
        )
        .route("/worklogs/filter", post(controllers::filter))
        .route(
            "/worklogs/export",
            get(controllers::export_query).post(controllers::export),
        )
        .route("/worklogs/{id}", delete(controllers::delete))
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}

async fn health() -> &'static str {
    "ok"
}

use axum::{
    middleware::from_fn_with_state,
    routing::{get, post},
    Router,
};
use tower_http::trace::TraceLayer;

use crate::middleware::require_auth;
use crate::routes::controllers;
use crate::state::AppState;

pub fn router(state: AppState) -> Router {
    let protected = Router::new()
        .route(
            "/worklogs",
            post(controllers::create).get(controllers::filter_query),
        )
        .route("/worklogs/filter", post(controllers::filter))
        .route(
            "/worklogs/export",
            get(controllers::export_query).post(controllers::export),
        )
        .route(
            "/worklogs/{id}",
            get(controllers::get).delete(controllers::delete),
        )
        .route_layer(from_fn_with_state(state.clone(), require_auth))
        .with_state(state.clone());

    Router::new()
        .route("/health", get(health))
        .merge(protected)
        .layer(TraceLayer::new_for_http())
}

async fn health() -> &'static str {
    "ok"
}

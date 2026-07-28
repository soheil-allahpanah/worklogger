use axum::{extract::State, Extension, Json};
use domain::actor::ActorContext;

use crate::dto::MeJson;
use crate::error::ApiResult;
use crate::state::AppState;

pub async fn me(
    State(state): State<AppState>,
    Extension(actor): Extension<ActorContext>,
) -> ApiResult<Json<MeJson>> {
    let response = state.get_me().execute(actor).await?;
    Ok(Json(MeJson::from_response(response)))
}

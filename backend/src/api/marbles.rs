use api::marble::Marble;
use axum::{Json, extract::State, http::StatusCode};
use database::marble::DbMarble;

use crate::{AppState, util::internal_server_error};

pub async fn get_marbles(
    State(state): State<AppState>,
) -> Result<Json<Vec<Marble>>, (StatusCode, String)> {
    let db_marbles = DbMarble::get_all_active(&state.pool)
        .await
        .map_err(internal_server_error)?;

    let marbles = db_marbles.into_iter().map(Marble::from).collect();

    Ok(Json(marbles))
}

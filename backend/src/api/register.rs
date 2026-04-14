use api::{RegisterRequest, User};
use axum::{Json, extract::State, http::StatusCode};
use database::user::DbUser;

use crate::{AppState, util::internal_server_error};

pub async fn post_register(
    State(state): State<AppState>,
    Json(register_request): Json<RegisterRequest>,
) -> Result<Json<User>, (StatusCode, String)> {
    let maybe_db_user = DbUser::get_by_username(&state.pool, &register_request.username)
        .await
        .map_err(internal_server_error)?;

    if maybe_db_user.is_some() {
        return Err((StatusCode::CONFLICT, String::from("username is taken")));
    }

    if register_request.password != register_request.confirm_password {
        return Err((
            StatusCode::BAD_REQUEST,
            String::from("passwords do not match"),
        ));
    }

    let db_user = DbUser::insert(
        &state.pool,
        &register_request.username,
        &register_request.password,
    )
    .await
    .map_err(internal_server_error)?;

    Ok(Json(User::from(db_user)))
}

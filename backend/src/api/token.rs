use axum::{Json, extract::State, http::StatusCode};
use database::user::DbUser;
use jwt::SignWithKey;
use api::{TokenClaims, TokenRequest};

use crate::{AppState, util::internal_server_error};

pub async fn post_token(
    State(state): State<AppState>,
    Json(token_request): Json<TokenRequest>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    let maybe_db_user = DbUser::get_by_username(&state.pool, &token_request.username)
        .await
        .map_err(internal_server_error)?;

    if let Some(db_user) = maybe_db_user
        && db_user.verify_password(&token_request.password)
    {
        let token_claims = TokenClaims { sub: db_user.id };

        let token = token_claims.sign_with_key(&state.key).unwrap();

        Ok((StatusCode::OK, token))
    } else {
        Err((
            StatusCode::UNAUTHORIZED,
            String::from("username or password not found"),
        ))
    }
}

use std::error::Error;

use axum::http::StatusCode;

pub fn internal_server_error(error: impl Error) -> (StatusCode, String) {
    (StatusCode::INTERNAL_SERVER_ERROR, format!("{error}"))
}

pub fn to_row_not_found<T>(maybe: Option<T>) -> sqlx::Result<T> {
    maybe.ok_or(sqlx::Error::RowNotFound)
}

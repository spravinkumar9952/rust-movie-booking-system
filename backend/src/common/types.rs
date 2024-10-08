use axum::{http::StatusCode, response::{IntoResponse, Response}, Json};
use serde::Serialize;

#[derive(Serialize)]
pub struct ErrorResponse {
  pub message: String,
  pub error_code: u16,
  pub description: String,
}

impl IntoResponse for ErrorResponse {
  fn into_response(self) -> Response {
      let status = match self.error_code {
          400 => StatusCode::BAD_REQUEST,
          401 => StatusCode::UNAUTHORIZED,
          403 => StatusCode::FORBIDDEN,
          404 => StatusCode::NOT_FOUND,
          500 => StatusCode::INTERNAL_SERVER_ERROR,
          _ => StatusCode::INTERNAL_SERVER_ERROR,
      };
      (status, Json(self)).into_response()
  }
}

#[derive(Serialize)]
pub struct SuccessResponse {
  pub message: String,
}

impl IntoResponse for SuccessResponse {
  fn into_response(self) -> Response {
      (StatusCode::OK, Json(self)).into_response()
  }
}

pub fn throw200<T>(message: &str) -> Result<SuccessResponse, T> {
  return Ok(SuccessResponse {
      message: message.to_string(),
  });
}
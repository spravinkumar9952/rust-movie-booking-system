use axum::{response::{IntoResponse, Response}, Json};
use serde::Serialize;

#[derive(Serialize)]
pub struct ErrorResponse {
  pub message: String,
  pub error_code: u16,
  pub description: String,
}

impl IntoResponse for ErrorResponse {
  fn into_response(self) -> Response {
      Json(self).into_response()
  }
}
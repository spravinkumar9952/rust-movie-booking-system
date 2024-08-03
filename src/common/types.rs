use serde::Serialize;

#[derive(Serialize)]
struct ErrorResponse {
  message: String,
  error_code: u16,
  description: String,
}
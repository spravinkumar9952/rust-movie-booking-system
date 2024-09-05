use crate::common::types::ErrorResponse;


pub fn throw500<T>( error: &str, message: &str) -> Result<T, ErrorResponse> {
  return Err(ErrorResponse {
      message: message.to_string(),
      error_code: 500,
      description: error.to_string(),
  });
}

pub fn throw400<T>( error: &str, message: &str) -> Result<T, ErrorResponse> {
  return Err(ErrorResponse {
      message: message.to_string(),
      error_code: 400,
      description: error.to_string(),
  });
}

pub fn throw401<T>( error: &str, message: &str) -> Result<T, ErrorResponse> {
  return Err(ErrorResponse {
      message: message.to_string(),
      error_code: 401,
      description: error.to_string(),
  });
}



use axum::{
  extract::{Extension, FromRequest, TypedHeader},
  headers::{authorization::Bearer, Authorization},
  http::{Request, StatusCode},
  middleware::Next,
  response::{IntoResponse, Response},
  Json, Router,
};
use sqlx::PgPool;
use std::sync::Arc;
use tower::{ServiceBuilder, layer::layer_fn};
use serde::Serialize;

#[derive(Serialize)]
struct ErrorResponse {
  message: String,
  error_code: u16,
  description: String,
}

pub async fn verify_normal_token<B>(
  req: Request<B>,
  next: Next<B>,
) -> Result<Response, StatusCode> {
  Ok(next.run(req).await)
  // let (mut parts, body) = req.into_parts();
  // let Extension(pool): Extension<Arc<PgPool>> = Extension::(&mut parts, &()).await.unwrap();
  // let TypedHeader(Authorization(token)): TypedHeader<Authorization<Bearer>> = TypedHeader::from_request_parts(&mut parts, &()).await.unwrap();

  // let table = "users";
  // let valid = validate_token(token.token(), table, &pool).await;
  // let req = Request::from_parts(parts, body);
  
  // if valid {
  //     Ok(next.run(req).await)
  // } else {
  //     let error_response = ErrorResponse {
  //         message: "Invalid or missing token".to_string(),
  //         error_code: 401,
  //         description: "Unauthorized".to_string(),
  //     };
  //     Ok((StatusCode::UNAUTHORIZED, Json(error_response)).into_response())
  // }
}


pub async fn verify_dashboard_token<B>(
  TypedHeader(Authorization(token)): TypedHeader<Authorization<Bearer>>,
  Extension(pool): Extension<Arc<PgPool>>,
  req: Request<B>,
  next: Next<B>,
) -> Result<Response, StatusCode> {
  Ok(next.run(req).await)
  // let table = "admin";
  // let valid = validate_token(token.token(), &table, &pool).await;
  // if valid {
  //     Ok(next.run(req).await)
  // } else {
  //     let error_response = ErrorResponse {
  //         message: "Invalid or missing token".to_string(),
  //         error_code: 401,
  //         description: "Unauthorized".to_string(),
  //     };
  //     Ok((StatusCode::UNAUTHORIZED, Json(error_response)).into_response())
  // }
}

async fn validate_token(token: &str, person: &str, pool: &PgPool) -> bool {
  let result = sqlx::query!(
      "SELECT * FROM users WHERE registration_token = $1",
      token,
  )
  .fetch_optional(pool)
  .await
  .unwrap();

  result.is_some()
}

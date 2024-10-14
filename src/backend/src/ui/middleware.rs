use axum::{
  body::Body, 
  http::{Request, StatusCode}, 
  middleware::Next,
  response::{IntoResponse, Response}
};
use sqlx::PgPool;
use std::{convert::Infallible, sync::Arc};


#[derive(Clone)]
struct TokenValidator;

pub async fn token_validator<B>(req: Request<B>, next: Next<B>) -> Result<Response, Infallible>
where
  B: Send + 'static,
{
  let mut authorized = false;
  let db_pool = req.extensions().get::<Arc<PgPool>>().unwrap();

  // Check for the token in the headers
  if let Some(token) = req.headers().get("token") {
      if let Ok(token_str) = token.to_str() {
         let query = sqlx::query!("SELECT * FROM users WHERE registration_token = $1", token_str);
          let result = query.fetch_one(&**db_pool).await;
          if result.is_ok() {
              authorized = true;
          }
      }
  }

  if authorized {
      Ok(next.run(req).await)
  } else {
      // Return 401 Unauthorized
      Ok(Response::builder()
            .status(StatusCode::UNAUTHORIZED)
            .body(Body::from("Unauthorized"))
            .unwrap()
            .into_response())
  }
}
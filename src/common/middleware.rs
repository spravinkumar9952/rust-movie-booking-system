use axum::{
  body::Body, 
  http::{Request, StatusCode}, 
  middleware::Next,
  response::{IntoResponse, Response}
};
use std::convert::Infallible;


#[derive(Clone)]
struct TokenValidator;

pub async fn token_validator<B>(req: Request<B>, next: Next<B>) -> Result<Response, Infallible>
where
  B: Send + 'static,
{
  let mut authorized = false;

  // Check for the token in the headers
  if let Some(token) = req.headers().get("token") {
      if let Ok(token_str) = token.to_str() {
          if token_str == "your_valid_token_here" {
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
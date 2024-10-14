use axum::{
  http::StatusCode, 
  response::{IntoResponse, Json as JsonResponse, Response},  
  Extension, 
  Json, 
};
use serde::Deserialize;
use serde::Serialize;
use sqlx::PgPool;
use std::sync::Arc;
use chrono::Utc;

use crate::utils;



#[derive(Deserialize)]
pub struct LoginCredentials {
  phone_number: String,
  password: String
}

#[derive(Serialize)]
pub struct LoginResponse {
  registration_token: String
}

#[derive(Serialize)]
pub struct ErrorResponse {
  message: String,
  error_code: i32,
  description: String 
}

pub async fn login(Json(payload): Json<LoginCredentials>, Extension(db_pool) : Extension<Arc<PgPool>>) -> Result<Json<LoginResponse>, Response>{
  let result = sqlx::query!(
      "SELECT phone_number FROM users WHERE phone_number = $1 AND password = $2",
      payload.phone_number,
      payload.password
  )
  .fetch_optional(&*db_pool)
  .await
  .unwrap();

  if result.is_some() {
      let registration_token: String = utils::generate_random_string();

      create_and_update_registration_token(&*db_pool, &payload.phone_number, &registration_token).await;
      Ok(Json(LoginResponse{
          registration_token : registration_token
      }))
  } else {
      Err((StatusCode::UNAUTHORIZED, Json(ErrorResponse{
          message: "Invalid credentials!".to_string(),
          error_code: 401,
          description: "Invalid credentials".to_string()
      })).into_response())
  }
}

#[derive(Serialize)]
pub struct RegisterResponse {
  message: String
}

pub async fn create_and_update_registration_token(db: &PgPool, phone_number: &String, registration_token: &String) {
  let result = sqlx::query!(
      "SELECT phone_number FROM users WHERE phone_number = $1",
      phone_number
  )
  .fetch_optional(&*db)
  .await;


  let created_at = Utc::now().naive_utc();

  match result {
      Ok(Some(_)) => {
          // Update the token
          sqlx::query!(
              "UPDATE users SET registration_token = $1, created_at = $2 WHERE phone_number = $3",
              registration_token,
              created_at,
              phone_number,
              
          )
          .execute(&*db)
          .await
          .unwrap();
      }
      Ok(None) => {
          // Create a new token
          sqlx::query!(
              "INSERT INTO users (phone_number, registration_token, created_at) VALUES ($1, $2, $3)",
              phone_number,
              registration_token,
              created_at
          )
          .execute(&*db)
          .await
          .unwrap();
      }
      Err(e) => {
          eprintln!("Failed to create or update registration token: {:?}", e);
      }
  }
}



pub async fn register(Json(payload): Json<LoginCredentials>, Extension(db_pool) : Extension<Arc<PgPool>>) -> JsonResponse<RegisterResponse>{
  match sqlx::query!(
      "SELECT phone_number FROM users WHERE phone_number = $1",
      payload.phone_number
  )
  .fetch_optional(&*db_pool)
  .await{
      Ok(Some(_)) => {
          return JsonResponse(RegisterResponse {
              message: "User already Exists!".to_string(),
          })
      }
      Ok(None) => {
          // Continue
      }
      Err(e) => {
          eprintln!("Failed to register user: {:?}", e);
          return JsonResponse(RegisterResponse {
              message: "Failed to register user.".to_string(),
          })
      }
  }

  match sqlx::query!(
      "INSERT INTO users (phone_number, password) VALUES ($1, $2)",
      payload.phone_number,
      payload.password
  )
  .execute(&*db_pool)
  .await {
      Ok(_) => {
          // Registration successful
          JsonResponse(RegisterResponse {
              message: "User registered successfully!".to_string(),
          })
      }
      Err(e) => {
          // Registration failed
          eprintln!("Failed to register user: {:?}", e);
          JsonResponse(RegisterResponse {
              message: "Failed to register user.".to_string(),
          })
      }
  }

}
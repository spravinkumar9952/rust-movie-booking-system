use std::sync::Arc;

use axum::{response::{IntoResponse, Response}, Extension, Json};
use sqlx::PgPool; 
use chrono::Utc;

use crate::{common, utils};

#[derive(serde::Deserialize)]
pub struct AdminLoginCredentials {
    phone_number: String,
    password: String
}

#[derive(serde::Serialize)]
pub struct AdminLoginResponse {
    message: String,
    registration_token: String
}

impl IntoResponse for AdminLoginResponse {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}

pub async fn login_admin(
    Json(payload): Json<AdminLoginCredentials>, 
    Extension(db_pool) : Extension<Arc<PgPool>>
) -> Result<AdminLoginResponse, common::types::ErrorResponse>{
    let result = sqlx::query!(
        "SELECT phone_number FROM admins WHERE phone_number = $1 AND password = $2",
        payload.phone_number,
        payload.password
    )
    .fetch_optional(&*db_pool)
    .await
    .unwrap();

    if result.is_some() {
        let registration_token: String = utils::generate_random_string();
        create_and_update_registration_token(&*db_pool, &payload.phone_number, &registration_token).await;
        Ok(AdminLoginResponse {
            message: "Login successful!".to_string(),
            registration_token
        })
    } else {
        Err(common::types::ErrorResponse {
            message: "Invalid credentials!".to_string(),
            error_code: 401,
            description: "Invalid credentials".to_string()
        })
    }
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
            sqlx::query!(
                "UPDATE admins SET registration_token = $1, created_at = $2 WHERE phone_number = $3",
                registration_token,
                created_at,
                phone_number,
            )
            .execute(&*db)
            .await
            .unwrap();
        }
        Ok(None) => {
            sqlx::query!(
                "INSERT INTO admins (phone_number, registration_token, created_at) VALUES ($1, $2, $3)",
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
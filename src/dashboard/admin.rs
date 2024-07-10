use std::sync::Arc;

use axum::{Extension, Json,  response::Json as JsonResponse};
use sqlx::PgPool; 
use rand::{distributions::Alphanumeric, Rng};
use chrono::{Utc, Duration};

#[derive(serde::Deserialize)]
pub struct AdminLoginCredentials {
    phone_number: String,
    password: String
}

#[derive(serde::Serialize)]
pub struct AdminLoginResponse {
    message: String
}

pub async fn login_admin(Json(payload): Json<AdminLoginCredentials>, Extension(db_pool) : Extension<Arc<PgPool>>) -> JsonResponse<AdminLoginResponse>{
    let result = sqlx::query!(
        "SELECT phone_number FROM admins WHERE phone_number = $1 AND password = $2",
        payload.phone_number,
        payload.password
    )
    .fetch_optional(&*db_pool)
    .await
    .unwrap();

    if result.is_some() {
        create_and_update_registration_token(&*db_pool, &payload.phone_number).await;
        JsonResponse(AdminLoginResponse {
            message: "Login successful!".to_string()
        })
    } else {
        JsonResponse(AdminLoginResponse {
            message: "Invalid credentials!".to_string()
        })
    }
}

async fn create_and_update_registration_token(db: &PgPool, phone_number: &String) {
    let registration_token: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(30)
        .map(char::from)
        .collect();
    let created_at = Utc::now().naive_utc();
    sqlx::query!(
        "UPDATE admins SET registration_token = $1, created_at = $2 WHERE phone_number = $3",
        registration_token,
        created_at,
        phone_number
    )
    .execute(&*db)
    .await
    .unwrap();
}
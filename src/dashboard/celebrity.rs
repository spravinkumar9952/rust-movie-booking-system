use std::sync::Arc;

use axum::{Extension, Json,  response::Json as JsonResponse,  extract::TypedHeader,
  headers::{Authorization, authorization::Bearer},
  http::StatusCode,};
use sqlx::{PgPool, Pool, Postgres}; 




#[derive(serde::Deserialize)]
pub struct AddCelebrity {
    name: String
}

#[derive(serde::Serialize)]
pub struct AddCelebrityResponse {
    message: String
}

pub async fn add_celebrity(TypedHeader(Authorization(token)): TypedHeader<Authorization<Bearer>>,Json(payload): Json<AddCelebrity>, Extension(db_pool): Extension<Arc<PgPool>>) -> JsonResponse<AddCelebrityResponse> {
    let user_id = validate_admin_token(token.token(), &db_pool).await.map_err(|_| StatusCode::UNAUTHORIZED);
    if user_id.is_err() {
        return JsonResponse(AddCelebrityResponse {
            message: "Unauthorized".to_string(),
        });
    }

    let result = sqlx::query!(
        "INSERT INTO celebrity (name) VALUES ($1)",
        payload.name
    )
    .execute(&*db_pool)
    .await;

    match result {
        Ok(_) => {
          JsonResponse(AddCelebrityResponse {
                message: "Celebrity added successfully!".to_string(),
            })
        }
        Err(e) => {
            eprintln!("Failed to add celebrity: {:?}", e);
            JsonResponse(AddCelebrityResponse {
                message: "Failed to add celebrity.".to_string(),
            })
        }
    }
  }


  async fn validate_admin_token(token: &str, pool: &Pool<Postgres>) -> Result<i32, sqlx::Error> {
    print!("token: {}", token);
    
    let result = sqlx::query!(
        "SELECT id FROM admins WHERE registration_token = $1",
        token
    )
    .fetch_optional(pool)
    .await?;

    if let Some(record) = result {
        Ok(record.id)
    } else {
        Err(sqlx::Error::RowNotFound)
    }
}
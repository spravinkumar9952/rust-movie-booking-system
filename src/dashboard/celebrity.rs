use std::sync::Arc;

use axum::{Extension, Json,  response::Json as JsonResponse,  extract::TypedHeader,
  headers::{HeaderMapExt, Authorization, authorization::Bearer},
  http::StatusCode,};
use sqlx::PgPool; 




#[derive(serde::Deserialize)]
pub struct AddCelebrity {
    name: String
}

#[derive(serde::Serialize)]
pub struct AddCelebrityResponse {
    message: String
}

pub async fn add_celebrity(TypedHeader(Authorization(token)): TypedHeader<Authorization<Bearer>>,Json(payload): Json<AddCelebrity>, Extension(db_pool): Extension<Arc<PgPool>>) -> JsonResponse<AddCelebrityResponse> {

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

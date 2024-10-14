use std::sync::Arc;
use axum::{
    response::IntoResponse, 
    Extension, 
    Json
};
use sqlx::PgPool;
use crate::common;

#[derive(serde::Deserialize)]
pub struct AddCelebrity {
    name: String
}

#[derive(serde::Serialize)]
pub struct AddCelebrityResponse {
    celeb_id: String
}

impl IntoResponse for AddCelebrityResponse {    
    fn into_response(self) -> axum::response::Response {
        Json(self).into_response()
    }
}

pub async fn add_celebrity(
    Json(payload): Json<AddCelebrity>,
    Extension(db_pool): Extension<Arc<PgPool>>
) -> Result<AddCelebrityResponse, common::types::ErrorResponse> {
    
    let celeb_id: String = uuid::Uuid::new_v4().to_string();
    let result = sqlx::query!(
        "INSERT INTO celebrity (name, id) VALUES ($1, $2)",
        payload.name,
        celeb_id
    )
    .execute(&*db_pool)
    .await;

    match result {
        Ok(_) => {
          Ok(AddCelebrityResponse {
                celeb_id
            })
        }
        Err(e) => {
            Err(common::types::ErrorResponse { 
                message: "Failed to add celebrity.".to_string(),
                error_code: 500,
                description: e.to_string(),
            })
        }
    }
  }
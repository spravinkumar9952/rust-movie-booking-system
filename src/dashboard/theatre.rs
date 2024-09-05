use core::str;
use std::sync::Arc;

use axum::{http::StatusCode, response::{IntoResponse, Response}, Extension, Json};
use sqlx::PgPool;

use crate::common::{error::throw500, types::ErrorResponse};


#[derive(serde::Serialize, serde::Deserialize)]
pub struct AddTheatreReq{
  name : String,
  address : String,
  no_of_screens : i32,
}


#[derive(serde::Deserialize, serde::Serialize)]
pub struct AddTheatreRes{
  message : String
}


impl IntoResponse for AddTheatreRes {
  fn into_response(self) -> Response {
      (StatusCode::OK, Json(self)).into_response()
  }
}

pub async fn add_theatre(
  Json(payload): Json<AddTheatreReq>,
  Extension(db_pool): Extension<Arc<PgPool>>
) -> Result<AddTheatreRes, ErrorResponse> {

  let result = sqlx::query!(
    "INSERT INTO theatre ( name, address, no_of_screens) VALUES ($1, $2, $3)",
    payload.name,
    payload.address,
    payload.no_of_screens
  )
  .execute(&*db_pool)
  .await;

  match result {
    Ok(_) => {
          Ok(AddTheatreRes {
            message: "Theatre added successfully.".to_string(),
          })
      }
    Err(e) => {
      return throw500("Failed to add theatre.", e.to_string().as_str());
    }
  }
}
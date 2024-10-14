use std::sync::Arc;
use axum::{Extension, Json};
use crate::common::{error::{throw400, throw500}, types::{throw200, ErrorResponse, SuccessResponse}};

use super::theatre;



#[derive(serde::Deserialize)]
pub struct AddTheatreSetasRequest {
  pub theatre_id: i32,
  pub screen_number: i32,
  pub row_number: i32,
  pub column_number_start : i32,
  pub column_number_end : i32,
}


pub async fn add_theatre_seats(
  Json(payload): Json<Vec<AddTheatreSetasRequest>>,
  Extension(shared_pool): Extension<Arc<sqlx::PgPool>>,
  Extension(_redis_client): Extension<Arc<redis::Client>>,
) -> Result<SuccessResponse, ErrorResponse> {
  // Create transactions
  let mut transaction = match shared_pool.begin().await {
    Ok(tx) => tx,
    Err(e) => {
      return Err(ErrorResponse {
        message: "Failed to start transaction.".to_string(),
        error_code: 500,
        description: e.to_string(),
      });
    }
  };

  // Check if theatre exists
  let theatre = sqlx::query!("SELECT * FROM theatre WHERE id = $1", payload[0].theatre_id)
    .fetch_one(&mut transaction)
    .await
    .expect("Failed to fetch theatre.");

  for(_, seat_row) in payload.iter().enumerate() {
  
    if seat_row.column_number_start > seat_row.column_number_end {
      transaction.rollback().await.unwrap_or_default();
      return throw400("Column number start should be less than column number end.", "Invalid column number.")
    }

    if seat_row.column_number_start < 1 || seat_row.column_number_end > theatre.column {
      transaction.rollback().await.unwrap_or_default();
      return throw400(&format!("Column number should be between 1 and {}.",  theatre.column), "Invalid column number.")
    }

    if seat_row.screen_number > theatre.no_of_screens  || seat_row.screen_number < 1 {
      transaction.rollback().await.unwrap_or_default();
      return throw400(&format!("Screen number should be between 1 and {}", theatre.no_of_screens), "Invalid Screen number.")
    }

    if seat_row.row_number <= 0 || seat_row.row_number > theatre.row {
      transaction.rollback().await.unwrap_or_default();
      return throw400(&format!("Row number should be between 1 and {}.", theatre.row), "Invalid row number.")
    }

    for col in seat_row.column_number_start..seat_row.column_number_end {

      if (sqlx::query!(
        "SELECT * FROM theatre_seat WHERE theatre_id = $1 AND \"row\" = $2 AND \"column\" = $3 AND screen_number = $4",
        seat_row.theatre_id,
        seat_row.row_number,
        col,
        seat_row.screen_number
      ).fetch_one(&mut transaction)
      .await
      .is_ok()){
        println!("Seat already exists.");
        return throw400("Seat already exists.", "Seat already exists.");
      }

      let result = sqlx::query!(
        "INSERT INTO theatre_seat (theatre_id, \"row\", \"column\", screen_number) VALUES ($1, $2, $3, $4)",
        seat_row.theatre_id,
        seat_row.row_number,
        col,
        seat_row.screen_number
      )
      .execute(&mut transaction)
      .await;

      match result {
        Ok(_) => {
          continue;
        }
        Err(e) => {
          transaction.rollback().await.unwrap_or_default();
          return throw500("Failed to add theatre seats.", &e.to_string());
        }
      }
    }
  }

  if let Err(e) = transaction.commit().await {
    return Err(ErrorResponse {
      message: "Failed to commit transaction.".to_string(),
      error_code: 500,
      description: e.to_string(),
    });
  }
  throw200("Theatre seats added successfully.")
}
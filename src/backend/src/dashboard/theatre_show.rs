use std::{result, sync::Arc};

use axum::{Extension, Json};
use chrono::NaiveDateTime;

use crate::common::{error::{throw400, throw500}, types::{ErrorResponse, SuccessResponse}};



pub struct AddShowRequest {
  pub theatre_id: i32,
  pub screen_number: i32,
  pub movie_id: String,
  pub start_time: NaiveDateTime,
  pub end_time: NaiveDateTime,
}


pub async fn add_shhows(
  Json(payload): Json<Vec<AddShowRequest>>,
  Extension(shared_pool): Extension<Arc<sqlx::PgPool>>,
  Extension(_redis_client): Extension<Arc<redis::Client>>,
) -> Result<SuccessResponse, ErrorResponse> {
  // Create transactions
  let mut transaction = match shared_pool.begin().await {
    Ok(tx) => tx,
    Err(e) => {
      return throw500( &"Failed to start transaction.".to_string(), &e.to_string());
    }
  };

  for(_, show) in payload.iter().enumerate() {
    let theatre = sqlx::query!("SELECT * FROM theatre WHERE id = $1", show.theatre_id)
      .fetch_one(&mut transaction)
      .await
      .expect("Failed to fetch theatre.");

    if show.start_time >= show.end_time {
      transaction.rollback().await.unwrap_or_default();
      return throw400(&"Start time should be less than end time.", &"Invalid time.")
    }

    if show.screen_number > theatre.no_of_screens  || show.screen_number < 1 {
      transaction.rollback().await.unwrap_or_default();
      return throw400(&format!("Screen number should be between 1 and {}", theatre.no_of_screens), "Invalid Screen number.")
    }

    let show_in_the_time_range = sqlx::query!("SELECT * FROM theatre_show WHERE theatre_id = $1 AND screen_number = $2 AND ((start_time > $3 AND start_time < $3) OR (end_time < $4 AND end_time > $4))",
      show.theatre_id,
      show.screen_number,
      show.start_time,
      show.end_time
      
    ).fetch_one(&mut transaction).await;

    if(show_in_the_time_range.is_ok()) {
      transaction.rollback().await.unwrap_or_default();
      return throw400("Show already exists for this time.", "Invalid time.")
    }

    let show_insert_result = sqlx::query!("INSERT INTO theatre_show (theatre_id, movie_id, screen_number, start_time, end_time) VALUES ($1, $2, $3, $4, $5)",
      show.theatre_id,
      show.movie_id,
      show.screen_number,
      show.start_time,
      show.end_time
    ).execute(&mut transaction).await;

    if(show_insert_result.is_err()) {
      transaction.rollback().await.unwrap_or_default();
      return throw500("Failed to add show.", &show_insert_result.unwrap_err().to_string())
    }
  }

  match (transaction.commit().await) {
    Ok(_) => {
      Ok(SuccessResponse {
        message: "Shows added successfully.".to_string(),
      })
    }
    Err(e) => {
      return throw500("Failed to commit transaction.", &e.to_string())
    }
  }
}
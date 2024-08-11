use std::sync::Arc;
use axum::{response::{IntoResponse, Response}, Extension, Json};
use sqlx::PgPool;
use uuid::Uuid;
use serde::{Serialize, Deserialize};

use crate::common::types::ErrorResponse;

#[derive(Debug, Serialize, Deserialize)]
enum MovieGenre {
    Action, Comedy, Drama, Horror, Romance
}

impl ToString for MovieGenre {
    fn to_string(&self) -> String {
        match self {
            MovieGenre::Action => "Action".to_string(),
            MovieGenre::Comedy => "Comedy".to_string(),
            MovieGenre::Drama => "Drama".to_string(),
            MovieGenre::Horror => "Horror".to_string(),
            MovieGenre::Romance => "Romance".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct AddMovieResponse {
    message: String,
}

impl IntoResponse for AddMovieResponse {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}

#[derive(Serialize, Deserialize)]
pub struct AddMovieRequest {
    title: String,
}

pub async fn add_movie(
    Json(payload): Json<AddMovieRequest>, 
    Extension(db_pool): Extension<Arc<PgPool>>
) -> Result<AddMovieResponse, ErrorResponse> {
    let movie_id = Uuid::new_v4();
    
    match sqlx::query!(
      "INSERT INTO movie (
        id, 
        title
        ) VALUES ($1, $2)",
      movie_id,
      payload.title
    )
    .execute(&*db_pool)
    .await {
        Ok(_) => {
            Ok(AddMovieResponse {
                message: "Movie added successfully!".to_string(),
            })
        }
        Err(e) => {
            Err(ErrorResponse {
                message: "Failed to add movie.".to_string(),
                error_code: 500,
                description: e.to_string(),
            })
        }
    }
}

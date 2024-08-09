use std::sync::Arc;
use axum::{Extension, Json};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;
use serde::{Serialize, Deserialize};

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

#[derive(Serialize, Deserialize)]
pub struct AddMovieRequest {
    title: String,
}

pub async fn add_movie(Json(payload): Json<AddMovieRequest>, Extension(db_pool): Extension<Arc<PgPool>>) -> Json<AddMovieResponse> {
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
        Ok(_) => Json(AddMovieResponse {
            message: "Movie added successfully.".to_string(),
        }),
        Err(e) => {
            eprintln!("Failed to add movie: {:?}", e);
            Json(AddMovieResponse {
                message: "Failed to add movie.".to_string(),
            })
        }
    }
}

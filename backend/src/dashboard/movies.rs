use std::sync::Arc;
use axum::{http::StatusCode, response::{IntoResponse, Response}, Extension, Json};
use sqlx::{PgPool, Postgres, Transaction};
use uuid::Uuid;
use serde::{Serialize, Deserialize};

use crate::common::{error::throw500, types::ErrorResponse};

#[derive(Debug, Serialize, Deserialize)]
enum MovieGenre {
    Action, Comedy, Drama, Horror, Romance, SciFi
}

impl ToString for MovieGenre {
    fn to_string(&self) -> String {
        match self {
            MovieGenre::Action => "Action".to_string(),
            MovieGenre::Comedy => "Comedy".to_string(),
            MovieGenre::Drama => "Drama".to_string(),
            MovieGenre::Horror => "Horror".to_string(),
            MovieGenre::Romance => "Romance".to_string(),
            MovieGenre::SciFi => "SciFi".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct AddMovieResponse {
    message: String,
}

impl IntoResponse for AddMovieResponse {
    fn into_response(self) -> Response {
        (StatusCode::OK, Json(self)).into_response()
    }
}

#[derive(Serialize, Deserialize)]
pub struct AddMovieRequest {
    title: String,
    genre: String,
    actors_id: Vec<String>
}

pub async fn add_movie(
    Json(payload): Json<AddMovieRequest>, 
    Extension(db_pool): Extension<Arc<PgPool>>
) -> Result<AddMovieResponse, ErrorResponse> {
    let movie_id = Uuid::new_v4();

    let mut transaction: Transaction<Postgres> = match db_pool.begin().await {
        Ok(tx) => tx,
        Err(e) => {
            return Err(ErrorResponse {
                message: "Failed to start transaction.".to_string(),
                error_code: 500,
                description: e.to_string(),
            });
        }
    };
    
    match sqlx::query!(
      "INSERT INTO movie (
        id, 
        title,
        genre
        ) VALUES ($1, $2, $3)",
      movie_id.to_string(),
      payload.title,
      payload.genre
    )
    .execute(&mut transaction)
    .await {
        Ok(_) => {
            for actor_id in &payload.actors_id {
                let res = sqlx::query!(
                    "INSERT INTO movie_celebrities (
                        movie_id, 
                        celebrity_id
                        ) VALUES ($1, $2)",
                    movie_id.to_string(),
                    actor_id
                )
                .execute(&mut transaction)
                .await;
                
                if res.is_err() {
                    transaction.rollback().await.unwrap_or_default();
                    return  Err(ErrorResponse {
                        message: "Failed to add movie.".to_string(),
                        error_code: 500,
                        description: res.unwrap_err().to_string(),
                    });
                }
            };

            if let Err(e) = transaction.commit().await {
                return Err(ErrorResponse {
                    message: "Failed to commit transaction.".to_string(),
                    error_code: 500,
                    description: e.to_string(),
                });
            }
            
            Ok(AddMovieResponse {
                message: "Movie added successfully!".to_string(),
            })
        }
        Err(e) => {
            transaction.rollback().await.unwrap_or_default();
            throw500("Failed to add movie", e.to_string().as_str())
        }
    }
}

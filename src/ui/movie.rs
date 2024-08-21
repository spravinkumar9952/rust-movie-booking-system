use std::sync::Arc;

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Extension, Json,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::common;

#[derive(Serialize, Deserialize)]
struct Celebrity {
    id: String,
    name: String,
}

#[derive(Serialize, Deserialize)]
pub struct GetMoviesResponse {
    id: String,
    title: String,
    actors: Vec<Celebrity>,
}

impl IntoResponse for GetMoviesResponse {
    fn into_response(self) -> Response {
        (StatusCode::OK, Json(self)).into_response()
    }
}

pub async fn get_movies(
    Extension(db_pool): Extension<Arc<PgPool>>,
) -> Result<Json<Vec<GetMoviesResponse>>, common::types::ErrorResponse> {
    let movies = sqlx::query!("SELECT * FROM movie")
        .fetch_all(&*db_pool)
        .await;

    let mut resp = vec![];

    match movies {
        Ok(movies) => {
            for movie in movies {
                let actors_ids = sqlx::query!(
                    "SELECT * FROM movie_celebrities WHERE movie_id = $1",
                    movie.id
                )
                .fetch_all(&*db_pool)
                .await;

                match actors_ids {
                    Ok(actors) => {
                        let mut celebs = vec![];
                        for actor_id in actors {
                            let actor = sqlx::query!("SELECT * FROM celebrity WHERE id = $1", actor_id.celebrity_id)
                                .fetch_one(&*db_pool)
                                .await
                                .unwrap();

                            celebs.push(Celebrity {
                                id: actor.id,
                                name: actor.name,
                            });
                        }
                        resp.push(GetMoviesResponse {
                            id: movie.id,
                            title: movie.title,
                            actors: celebs,
                        });
                    }
                    Err(e) => {
                        return Err(common::types::ErrorResponse {
                            message: "Failed to fetch actors.".to_string(),
                            error_code: 500,
                            description: e.to_string(),
                        });
                    }
                }
            }
            Ok(axum::Json(resp))
        }
        Err(e) => {
            return Err(common::types::ErrorResponse {
                message: "Failed to fetch movies.".to_string(),
                error_code: 500,
                description: e.to_string(),
            });
        }
    }
}

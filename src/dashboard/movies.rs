
struct AddMovie {
    movie: Movie,
}

async fn add_movie(Json(payload): Json<AddMovie>, Extension(db_pool): Extension<Arc<PgPool>>) -> Json<AddMovieResponse> {
    let movie_id = Uuid::new_v4();
    let movie = payload.movie;
    

    match sqlx::query!(
      "INSERT INTO movies (movie_id, title) VALUES ($1, $2)",
      payload.movie_id,
      payload.title,
    )
    .execute(&*db_pool)
    .await {
        Ok(_) => {
          Json(AddMovieResponse {
                message: "Movie added successfully!".to_string(),
            })
        }
        Err(e) => {
            eprintln!("Failed to add movie: {:?}", e);
            Json(AddMovieResponse {
                message: "Failed to add movie.".to_string(),
            })
        }
    }
}
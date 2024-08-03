


async pub fn get_movie_list(TypedHeader(Authorization(token)): TypedHeader<Authorization<Bearer>>, Extension(db_pool): Extension<Arc<PgPool>>) -> Json<Vec<Movie>> {
    let movies = match sqlx::query_as::<_, Movie>("SELECT * FROM movies")
        .fetch_all(&*db_pool)
        .await {
            Ok(movies) => movies,
            Err(e) => {
                eprintln!("Failed to fetch movies: {:?}", e);
                Vec::new()
            }
        };

    Json(movies)
}
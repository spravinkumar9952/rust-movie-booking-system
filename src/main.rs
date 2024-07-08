use axum::{
    routing::{get, post}, Extension, Json, Router,
    response::Json as JsonResponse,
};
use std::net::SocketAddr;
use tokio;
use serde::Deserialize;
use serde::Serialize;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::sync::Arc;
use std::env;
use dotenv::dotenv;

// handler function for the root path
async fn root() -> &'static str {
    "Hello, Welcome to Rust Movie booking system!"
}

#[derive(Deserialize)]
struct LoginCredentials {
    phone_number: String,
    password: String
}

async  fn login(Json(payload): Json<LoginCredentials>, Extension(db_pool) : Extension<Arc<PgPool>>) -> &'static str{
    let result = sqlx::query!(
        "SELECT phone_number FROM users WHERE phone_number = $1 AND password = $2",
        payload.phone_number,
        payload.password
    )
    .fetch_optional(&*db_pool)
    .await
    .unwrap();

    if result.is_some() {
        "Login successful!"
    } else {
        "Invalid credentials!"
    }
}

#[derive(Serialize)]
struct RegisterResponse {
    message: String
}

async fn register(Json(payload): Json<LoginCredentials>, Extension(db_pool) : Extension<Arc<PgPool>>) -> JsonResponse<RegisterResponse>{
    let result = sqlx::query!(
        "INSERT INTO users (phone_number, password) VALUES ($1, $2)",
        payload.phone_number,
        payload.password
    )
    .execute(&*db_pool)
    .await;

    match result {
        Ok(_) => {
            // Registration successful
            JsonResponse(RegisterResponse {
                message: "User registered successfully!".to_string(),
            })
        }
        Err(e) => {
            // Registration failed
            eprintln!("Failed to register user: {:?}", e);
            JsonResponse(RegisterResponse {
                message: "Failed to register user.".to_string(),
            })
        }
    }

}


#[tokio::main]
async fn main() {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create pool.");

    let shared_pool = Arc::new(db_pool);

    // build our application with a single route
    let app = Router::new()
        .route("/", get(root))
        .route("/register", post(register))
        .route("/login", post(login))
        .layer(Extension(shared_pool));

    // specify the address and port for the server to listen on
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on http://{}", addr);

    // run the server
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

use axum::{
    middleware, 
    routing::{get, post}, 
    Extension, 
    Router
};

use std::net::SocketAddr;
use tokio;
use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;
use std::env;
use dotenv::dotenv;
use tower::ServiceBuilder;
use tower_http::cors::{CorsLayer, Any};

pub mod dashboard;
pub mod utils;
pub mod common;
pub mod ui;


async fn root() -> &'static str {
    "Hello, Welcome to Rust Movie booking system!"
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

    let redis_client =Arc::new(redis::Client::open("redis://127.0.0.1/").expect("Failed to create redis client."));

    let cors = CorsLayer::new()
        .allow_origin(Any)         // Allow any origin
        .allow_methods(Any)        // Allow any HTTP methods
        .allow_headers(Any);

    let ui_api: Router = Router::new()
        .route("/", get(root))
        .route("/movies", get(ui::movie::get_movies))
        .route("/theatre/list", get(ui::theatre::get_theatres))
        .layer(
            ServiceBuilder::new()
                .layer(middleware::from_fn(ui::middleware::token_validator)) 
        )
        .route("/register", post(ui::user::register))
        .route("/login", post(ui::user::login));
    
    let dashboard_api = Router::new()
        .route("/celebrity/add", post(dashboard::celebrity::add_celebrity))
        .route("/movie/add", post(dashboard::movies::add_movie))
        .route("/theatre/add", post(dashboard::theatre::add_theatre))
        .route("/theatre/seat/add", post(dashboard::theatre_seats::add_theatre_seats))
        .layer(
            ServiceBuilder::new()
                .layer(middleware::from_fn(dashboard::middleware::token_validator))
        )
        .route("/admin/login", post(dashboard::admin::login_admin));

    let app = Router::new()
        .nest("/", ui_api)
        .nest("/dashboard", dashboard_api)
        .layer(Extension(shared_pool))
        .layer(Extension(redis_client))
        .layer(Extension(cors));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on http://{}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

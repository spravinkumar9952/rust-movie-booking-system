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

    let ui_api: Router = Router::new()
        .route("/", get(root))
        .layer(
            ServiceBuilder::new()
                .layer(middleware::from_fn(ui::middleware::token_validator)) 
        )
        .route("/register", post(ui::user::register))
        .route("/login", post(ui::user::login))
        .route("/movies", get(ui::movie::get_movies));
    
    let dashboard_api = Router::new()
        .route("/celebrity/add", post(dashboard::celebrity::add_celebrity))
        .route("/movie/add", post(dashboard::movies::add_movie))
        .layer(
            ServiceBuilder::new()
                .layer(middleware::from_fn(dashboard::middleware::token_validator))
        )
        .route("/admin/login", post(dashboard::admin::login_admin));

    let app = Router::new()
        .nest("/", ui_api)
        .nest("/dashboard", dashboard_api)
        .layer(Extension(shared_pool));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on http://{}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

use axum::{
    http::StatusCode, middleware, response::{IntoResponse, Json as JsonResponse, Response}, routing::{get, post}, Extension, Json, Router
};
use core::time;
use std::net::SocketAddr;
use tokio;
use serde::Deserialize;
use serde::Serialize;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::sync::Arc;
use std::env;
use dotenv::dotenv;
use rand::{distributions::Alphanumeric, Rng};
use chrono::{Utc, Duration};
use tower::{ServiceBuilder, layer::layer_fn};

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

    let ui_api = Router::new()
        .route("/", get(root))
        .route("/register", post(ui::user::register))
        .route("/login", post(ui::user::login))
        .layer(
            ServiceBuilder::new()
                .layer(Extension(shared_pool.clone()))
                .layer(middleware::from_fn(common::middleware::token_validator))
        );
    
    let dashboard_api = Router::new()
        .route("/celebrity/add", post(dashboard::celebrity::add_celebrity))
        .route("/admin/login", post(dashboard::admin::login_admin))
        .route("/movies/add", post(dashboard::movies::add_movie));

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

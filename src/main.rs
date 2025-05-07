use axum::{
    routing::{get, post}, Router, Server
};
use sqlx::postgres::PgPoolOptions;
use std::env;
use dotenv::dotenv;

mod auth;
mod database;
mod model;
mod password;

use crate::auth::*;
use crate::database::init_database;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create pool.");

    //init database
    init_database(&pool).await.unwrap();

    //routes
    let app = Router::new()
    .route("/create_user", post(sign_up))
    .route("/login", post(authorizate))
    .route("/protected", get(protected))
    .with_state(pool);

    Server::bind(&"0.0.0.0:9487".parse().unwrap())
    .serve(app.into_make_service())
    .await
    .unwrap();
    
}
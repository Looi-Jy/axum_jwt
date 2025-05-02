use axum::{
    body::Body,
    response::IntoResponse,
    extract::{ Json, State},
    http::{Response, StatusCode},
};
use response::AuthError;

use crate::model::*;
use crate::database::*;
use crate::password::*;
use crate::encode::encode_jwt;

mod decode;
pub(crate) mod encode;
mod request;
mod response;

pub async fn sign_up(State(pool): State<sqlx::PgPool>, Json(user): Json<User>) -> impl IntoResponse {
    let hash_pwd = hash_password(&user.password).unwrap();
    match create_user(&pool, &user.name, &user.email, &hash_pwd).await {
        Ok(_) => {
            Response::builder()
            .status(StatusCode::CREATED)
            .body(Body::from("User created successfully"))
            .unwrap()
        }
        Err(err) => {
            Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(Body::from(format!("Failed to create user: {}", err)))
            .unwrap()
        }
    }
} 

pub async fn authorizate(State(pool): State<sqlx::PgPool>, Json(payload): Json<AuthPayload>) -> Result<Json<AuthBody>, AuthError> {
    // Check if the user sent the credentials
    if payload.email.is_empty() || payload.password.is_empty() {
        return Err(AuthError::MissingCredentials);
    } 

    // Attempt to retrieve user from DB
    let user = match get_user(&pool, &payload.email).await {
        Ok(user) => user,
        Err(err) => {
            println!("get_user failed: {}", err);
            return Err(AuthError::WrongCredentials);
        }
    };

    //Verify passord
    if !verify_password(&payload.password, &user.password)
    .map_err(|_| AuthError::WrongCredentials)? {
        return Err(AuthError::WrongCredentials);
    }

    let token = encode_jwt(user.name, user.email).map_err(|_| AuthError::TokenCreation)?;
    Ok(Json(AuthBody::new(token)))
}

pub async fn protected(claims: Claims) -> String {
    format!("Welcome to Rust, {}!", claims.name)
}
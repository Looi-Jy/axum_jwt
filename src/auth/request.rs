use axum::headers::authorization::Bearer;
use axum::headers::Authorization;
use axum::TypedHeader;
use axum::{ async_trait, extract::FromRequestParts, http::request::Parts, RequestPartsExt };
use crate::model::*;
use crate::auth::response::*;
use crate::auth::decode::*;

#[async_trait]
impl<S> FromRequestParts<S> for Claims where S: Send + Sync {
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // Extract the token from the authorization header
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>().await
            .map_err(|_| AuthError::InvalidToken)?;
        // Decode the user data
        let token_data = decode_jwt(bearer.token().to_string()).map_err(|_| AuthError::InvalidToken)?;
        Ok(token_data.claims)
    }
}
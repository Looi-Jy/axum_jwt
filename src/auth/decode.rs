use jsonwebtoken::{decode, errors::Error, Algorithm, DecodingKey, TokenData, Validation};
use crate::model::*;

pub fn decode_jwt(token: String) -> Result<TokenData<Claims>, Error> {
    let result = decode::<Claims>(&token, &DecodingKey::from_rsa_pem(include_bytes!("keys/public_key.pem"))?, &Validation::new(Algorithm::RS256))?;
    return Ok(result);
} 
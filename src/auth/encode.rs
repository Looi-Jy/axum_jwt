use chrono::{Duration, Utc};
use jsonwebtoken::{encode, errors::Error, Algorithm, EncodingKey, Header};
use crate::model::*;

pub fn encode_jwt(name: String, email: String) -> Result<String, Error> {
    let now = Utc::now();
    let expire: chrono::TimeDelta = Duration::hours(1);
    let exp: usize = (now + expire).timestamp() as usize;
    let iat: usize = now.timestamp() as usize;
    let claim = Claims { iat, exp, name, email };

    let token = encode(&Header::new(Algorithm::RS256), &claim, &EncodingKey::from_rsa_pem(include_bytes!("keys/private_key.pem"))?)?;
    return Ok(token);
}
use chrono::{Duration, Utc};
use jsonwebtoken::{self, DecodingKey, EncodingKey, Header, Validation};
use poem::error::{BadRequest, Unauthorized};
use serde::{Deserialize, Serialize};

// Token lifetime and Secret key are hardcoded for clarity
pub(crate) const JWT_EXPIRATION_HOURS: i64 = 2400;
const SECRET: &str = "SECRET";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct Claims {
    pub username: String,
    pub exp: i64,
}

impl Claims {
    pub fn new(username: String) -> Self {
        Self {
            username,
            exp: (Utc::now() + Duration::try_hours(JWT_EXPIRATION_HOURS).unwrap()).timestamp(),
        }
    }
}

/// Create a json web token (JWT)
pub(crate) fn create_jwt(claims: Claims) -> poem::Result<String> {
    let encoding_key = EncodingKey::from_secret(SECRET.as_bytes());
    jsonwebtoken::encode(&Header::default(), &claims, &encoding_key).map_err(BadRequest)
}

/// Decode a json web token (JWT)
pub(crate) fn decode_jwt(token: &str) -> poem::Result<Claims> {
    let decoding_key = DecodingKey::from_secret(SECRET.as_bytes());
    jsonwebtoken::decode::<Claims>(token, &decoding_key, &Validation::default())
        .map(|data| data.claims)
        .map_err(Unauthorized)
}

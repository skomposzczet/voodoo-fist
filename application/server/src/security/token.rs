use crate::model::user::User;
use serde::{Deserialize, Serialize};
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, decode, Header, EncodingKey};
use super::Error;
use dotenv;

const TOKEN_DURATION: i64 = 60;
const SECRET_KEY: &str = "SECRET";

#[derive(Debug, Deserialize, Serialize)]
struct Claims {
    sub: String,
    exp: usize,
}

impl Claims {
    fn from_user(user: &User) -> Claims {
        Claims {
            sub: format!("{:?}", user.id()),
            exp: Utc::now()
                .checked_add_signed(Duration::seconds(TOKEN_DURATION))
                .expect("Token expiration date exceeded")
                .timestamp() as usize
        }
    }
}

pub fn create_jwt(user: &User) -> Result<String, Error> {
    let claims = Claims::from_user(&user);
    encode(
        &Header::default(), 
        &claims, 
        &EncodingKey::from_secret(get_secret()?.as_bytes())
    ).map_err(|_| Error::JWTTokenCreationError)
}

fn get_secret() -> Result<String, Error> {
    dotenv::var(SECRET_KEY).map_err(|_| Error::MissingSecretKey)
}
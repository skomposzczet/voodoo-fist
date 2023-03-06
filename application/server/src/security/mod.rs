pub mod hash;
pub mod token;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("jwt token creation error")]
    JWTTokenCreationError,
    #[error("jwt token decode error")]
    JWTTokenDecodeError,
    #[error("missing secret key environment variable")]
    MissingSecretKey
}
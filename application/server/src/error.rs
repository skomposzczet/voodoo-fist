use thiserror::Error;
use warp::reject::Reject;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    JTTokenError( #[from] jsonwebtoken::errors::Error ),
    #[error(transparent)]
    EnvError( #[from] dotenv::Error ),
}

impl Reject for Error{}
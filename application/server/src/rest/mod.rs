pub mod user;
pub mod todo;

use serde::Serialize;
use serde_json::json;
use warp::{reject::Reject, reply::Json, Rejection};

#[derive(Debug)]
enum Error{
    NotUniqueError,
    InnerError,
    Unauthorized,
    NoUserWithSuchEmail,
    InvalidHeader
}
impl Reject for Error {}

fn json_response<T: Serialize>(data: &T) -> Result<Json, Rejection> {
    let response = json!({"data": data});
    Ok(warp::reply::json(&response))
}
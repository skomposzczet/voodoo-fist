pub mod user;
pub mod todo;
pub mod todo_item;

use std::sync::Arc;

use serde::Serialize;
use serde_json::json;
use warp::{reject::Reject, reply::Json, Rejection, Filter, hyper::HeaderMap, http::HeaderValue};

use crate::{security::token::{jwt_from_header, decode_jwt}, model::Db};

#[derive(Debug)]
enum Error{
    InnerError,
    Unauthorized,
    NoUserWithSuchEmail,
    InvalidHeader,
    BodyError(&'static str),
}
impl Reject for Error {}

pub fn routes(db: Arc<Db>) -> impl Filter<Extract = (impl warp::Reply,), Error = Rejection> + Clone {
    user::account_paths(db.clone())
        .or(todo::todo_list_paths(db.clone()))
        .or(todo_item::todo_item_paths(db.clone()))
}

fn json_response<T: Serialize>(data: &T) -> Result<Json, Rejection> {
    let response = json!({"data": data});
    Ok(warp::reply::json(&response))
}

fn with_auth() -> impl Filter<Extract = (String,), Error = Rejection> + Clone {
    warp::header::headers_cloned()
        .and_then(auth)
}

async fn auth(auth_header: HeaderMap<HeaderValue>) -> Result<String, warp::Rejection> {
    let token = jwt_from_header(&auth_header)
        .ok_or(Error::InvalidHeader)?;

    let token_data = decode_jwt(&token)?;

    Ok(token_data.claims.sub())
}
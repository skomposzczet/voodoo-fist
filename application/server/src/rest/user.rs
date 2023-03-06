use warp::{Filter, reject::Reject, Rejection};
use std::sync::Arc;
use crate::model::{Db, user::User};
use serde::Deserialize;
use crate::security::hash::hashed_password;

#[derive(Debug)]
enum Error{
    NotUniqueError,
    InnerError
}
impl Reject for Error {}

#[derive(Deserialize, Debug)]
struct RegisterBody {
    email: String,
    username: String,
    password: String
}

pub fn account_paths(db: Arc<Db>) -> impl Filter<Extract = impl warp::Reply, Error = Rejection> + Clone {
    let with_db = warp::any().map(move || db.clone());

    let register = warp::path("register")
        .and(warp::post())
        .and(warp::path::end())
        // .and(warp::any().map(move || db.clone()))
        .and(with_db.clone())
        .and(warp::body::json())
        .and_then(register_handle);

    register
}

async fn register_handle(db: Arc<Db>, body: RegisterBody) -> Result<String, Rejection> {
    if !is_unique_email(&db, &body.email).await {
        return Err(warp::reject::custom(Error::NotUniqueError));
    }
    let new_user = User::new(&body.email, &body.username, &hashed_password(&body.password));
    User::add_to_db(&db, &new_user).await.map_err(|_| Error::InnerError)?;

    Ok(String::from("Successfully added new user"))
}

async fn is_unique_email(db: &Db, email: &String) -> bool {
    true
}
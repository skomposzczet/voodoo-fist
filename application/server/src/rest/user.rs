use warp::{Filter, reject::Reject, Rejection, reply::Json};
use std::sync::Arc;
use crate::model::{Db, user::User};
use serde::{Deserialize, Serialize};
use crate::security::{hash::hashed_password, token};

#[derive(Debug)]
enum Error{
    NotUniqueError,
    InnerError,
    Unauthorized,
    NoUserWithSuchEmail
}
impl Reject for Error {}

#[derive(Deserialize, Debug)]
struct RegisterBody {
    email: String,
    username: String,
    password: String
}

#[derive(Deserialize, Debug)]
struct LoginBody {
    email: String,
    password: String
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub jwtoken: String,
}

pub fn account_paths(db: Arc<Db>) -> impl Filter<Extract = (impl warp::Reply,), Error = Rejection> + Clone {
    let with_db = warp::any().map(move || db.clone());

    let register = warp::path("register")
        .and(warp::post())
        .and(warp::path::end())
        .and(with_db.clone())
        .and(warp::body::json())
        .and_then(register_handle);

    let login = warp::path("login")
        .and(warp::post())
        .and(warp::path::end())
        .and(with_db.clone())
        .and(warp::body::json())
        .and_then(login_handle);

    register.or(login)
}

async fn login_handle(db: Arc<Db>, body: LoginBody) -> Result<Json, Rejection> {
    let user = User::get_by_email(&db, &body.email).await
        .map_err(|_| warp::reject::custom(Error::NoUserWithSuchEmail))?;
    if !user.password_matches(&hashed_password(&body.password)) {
        return Err(warp::reject::custom(Error::Unauthorized));
    }
    let token = token::create_jwt(&user)
        .map_err(|_| Error::InnerError)?;
    Ok(warp::reply::json( &LoginResponse {jwtoken: token}))
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
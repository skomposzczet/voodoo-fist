use warp::{Filter, reject::Reject, Rejection, reply::Json};
use std::sync::Arc;
use crate::model;
use crate::model::{Db, user::User};
use serde::{Deserialize, Serialize};
use serde_json::json;
use crate::security::{hash::hashed_password, token};
use std::str::FromStr;
use crate::rest::{Error, json_response, with_auth};

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

    let register = warp::path("register").and(warp::path::end())
        .and(warp::post())
        .and(with_db.clone())
        .and(warp::body::json())
        .and_then(register_handle);

    let login = warp::path("login").and(warp::path::end())
        .and(warp::post())
        .and(with_db.clone())
        .and(warp::body::json())
        .and_then(login_handle);

    let dashboard = warp::path("dashboard").and(warp::path::end())
        .and(warp::get())
        .and(with_db.clone())
        .and(with_auth())
        .and_then(dashboard_handle);

    register.or(login).or(dashboard)
}

async fn login_handle(db: Arc<Db>, body: LoginBody) -> Result<Json, Rejection> {
    let user = User::get_by_email(&db, &body.email).await
        .map_err(|_| warp::reject::custom(Error::NoUserWithSuchEmail))?;
    if !user.password_matches(&hashed_password(&body.password)) {
        return Err(warp::reject::custom(Error::Unauthorized));
    }
    let token = token::create_jwt(&user)
        .map_err(|_| Error::InnerError)?;
    json_response(&LoginResponse {jwtoken: token})
}

async fn register_handle(db: Arc<Db>, body: RegisterBody) -> Result<Json, Rejection> {
    let is_unique = is_unique_email(&db, &body.email).await?;
    if !is_unique {
        let response = json!({"error": "Email already used"});
        return json_response(&response);
    }
    let new_user = User::new(&body.email, &body.username, &hashed_password(&body.password));
    User::add_to_db(&db, &new_user).await.map_err(|_| Error::InnerError)?;

    json_response(&"Success")
}

async fn is_unique_email(db: &Db, email: &String) -> Result<bool, Error> {
    match User::get_by_email(db, &email).await {
        Ok(_) => Ok(false),
        Err(err) => {
            match err {
                model::Error::NoUserWithSuchEmail => Ok(true),
                _ => Err(Error::InnerError)
            }
        }
    }
}

async fn dashboard_handle(db: Arc<Db>, id: String) -> Result<Json, Rejection> {
    let user = User::get_by_id(&db, &id).await
        .map_err(|_| Error::InnerError)?;

    let response = json!({"username": user.username()});
    json_response(&response)
}

extern crate pretty_env_logger;

mod model;
mod security;
mod rest;
mod error;

use std::error::Error;
use std::sync::Arc;
use warp::Filter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    pretty_env_logger::init();
    println!("Connecting do db...");
    let db = Arc::new(model::db::init_db().await);
    println!("Successfully connected to db.");

    let cors = warp::cors()
        .allow_any_origin();

    let log = warp::log("server::vf");

    let routes = rest::routes(db.clone())
        .with(cors)
        .with(log);

    warp::serve(routes)
        .run(([0, 0, 0, 0], 8000)).await;
    Ok(())
}

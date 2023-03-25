extern crate pretty_env_logger;
#[macro_use] extern crate log;

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

    info!("Connecting do db...");
    let db = Arc::new(model::db::init_db().await);
    info!("Successfully connected to db.");

    let cors = warp::cors()
        .allow_any_origin();

    let log = warp::log("server::vf");

    let routes = rest::routes(db.clone())
        .with(cors)
        .with(log);

    info!("Starting server...");
    warp::serve(routes)
        .run(([0, 0, 0, 0], 8000)).await;
    Ok(())
}

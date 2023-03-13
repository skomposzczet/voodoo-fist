mod model;
mod security;
mod rest;
mod error;

use std::error::Error;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("Connecting do db...");
    let db = Arc::new(model::db::init_db().await);
    println!("Successfully connected to db.");

    warp::serve(rest::routes(db.clone()))
        .run(([127, 0, 0, 1], 8000)).await;
    Ok(())
}

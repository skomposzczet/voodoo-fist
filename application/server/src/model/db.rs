use mongodb::{Client, options::{ClientOptions}};
use std::error::Error;
use dotenv;

const MONGO_USER: &str = "MONGO_USER";
const MONGO_PW: &str = "MONGO_PW";
const MONGO_HOST: &str = "MONGO_HOST";
const MONGO_PORT: &str = "MONGO_PORT";

pub async fn init_db() -> Client {
    let client_uri = make_client_uri().unwrap();
    let options = ClientOptions::parse(&client_uri).await.unwrap();
    Client::with_options(options).unwrap()
}

fn make_client_uri() -> Result<String, Box<dyn Error>> {
    dotenv::dotenv()?;
    let user = dotenv::var(MONGO_USER)?;
    let password = dotenv::var(MONGO_PW)?;
    let host = dotenv::var(MONGO_HOST)?;
    let port = dotenv::var(MONGO_PORT)?;
    Ok(format!("mongodb://{}:{}@{}:{}", user, password, host, port))
}
pub mod db;
pub mod user;

pub use db::Db;

#[derive(Debug)]
pub enum Error {
    BsonError,
    DbError(&'static str),
    NoUserWithSuchEmail,
    CouldNotConnectToDB
}
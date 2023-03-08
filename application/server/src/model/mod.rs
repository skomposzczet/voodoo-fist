pub mod db;
pub mod user;
pub mod list;

pub use db::Db;

#[derive(Debug)]
pub enum Error {
    BsonError,
    DbError(&'static str),
    NoUserWithSuchEmail,
    CouldNotConnectToDB,
    InvalidUserID
}
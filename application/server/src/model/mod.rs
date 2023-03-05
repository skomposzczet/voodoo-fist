pub mod db;
pub mod user;

pub use db::Db;

pub enum Error {
    BsonError,
    DbError(&'static str)
}
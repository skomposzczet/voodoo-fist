pub mod db;
pub mod user;
pub mod list;

pub use db::Db;

use std::str::FromStr;
use bson::oid::ObjectId;

#[derive(Debug)]
pub enum Error {
    BsonError,
    DbError(&'static str),
    NoUserWithSuchEmail,
    CouldNotConnectToDB,
    InvalidOID,
    InvalidUserID,
}

pub fn objectid_from_str(id: &str) -> Result<ObjectId, Error> {
    mongodb::bson::oid::ObjectId::from_str(&id[10..34])
        .map_err(|_| Error::InvalidOID)
}
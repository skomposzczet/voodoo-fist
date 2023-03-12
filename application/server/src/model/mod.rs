pub mod db;
pub mod user;
pub mod list;
pub mod todo_item;

pub use db::Db;

use serde::de::DeserializeOwned;
use std::str::FromStr;
use bson::{oid::ObjectId, Document, Bson};

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

pub fn from_document<T: DeserializeOwned>(document: Document) -> Result<T, Error> {
    bson::from_bson(Bson::Document(document)).map_err(|_| Error::BsonError)
}
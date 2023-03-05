use serde::{Serialize, Deserialize};
use mongodb::bson::{Document, Bson, oid::ObjectId};
use std::error::Error;

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<ObjectId>,
    email: String,
    username: String,
    password: String
}

impl User {
    pub fn new(email: &String, username: &String, password: &String) -> User {
        User {id: None, email: email.clone(), username: username.clone(), password: password.clone() }
    }

    pub fn from_document(document: Document) -> Option<User> {
        bson::from_bson(Bson::Document(document)).ok()
    }
}
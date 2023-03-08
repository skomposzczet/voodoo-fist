use bson::doc;
use serde::{Serialize, Deserialize};
use mongodb::bson::{Document, Bson, oid::ObjectId};
use crate::model::{Db, db, Error};
use std::str::FromStr;

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

    pub fn id(self: &Self) -> Option<&ObjectId> {
        self.id.as_ref()
    }

    pub fn username(self: &Self) -> &String {
        &self.username
    }

    pub fn password_matches(self: &Self, hashed_password: &String) -> bool {
        self.password.eq(hashed_password)
    }

    pub async fn add_to_db(db: &Db, user: &User) -> Result<(), Error> {
        let bs = bson::to_bson(&user).map_err(|_| Error::BsonError)?;
        let document = bs.as_document().unwrap();
        let userdb = db.database("voodoofist").collection::<mongodb::bson::Document>("user");
        userdb.insert_one(document.to_owned(), None).await.map_err(|_| Error::DbError("Failed inserting item"))?;
        Ok(())
    }

    pub async fn get_by_email(db: &Db, email: &String) -> Result<User, Error> {
        let filter = doc!{"email": email.as_str()};
        let document = db::get_by(db, &filter, &String::from("user"))
            .await?
            .ok_or(Error::NoUserWithSuchEmail)?;
        
        let user = User::from_document(document)
            .ok_or(Error::BsonError)?;

        Ok(user)
    }

    pub async fn get_by_id(db: &Db, id: &String) -> Result<User, Error> {
        let id = &mongodb::bson::oid::ObjectId::from_str(&id[10..34])
            .map_err(|_| Error::InvalidUserID)?;
        let filter = doc!{"_id": id};

        let document = db::get_by(db, &filter, &String::from("user"))
            .await?
            .ok_or(Error::DbError("Couldnt fetch user:"))?;

        let user = User::from_document(document)
            .ok_or(Error::BsonError)?;
        Ok(user)
    }

}
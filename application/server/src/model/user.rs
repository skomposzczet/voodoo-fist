use bson::doc;
use serde::{Serialize, Deserialize};
use mongodb::bson::{Document, Bson, oid::ObjectId};
use crate::model::Db;
use crate::model;

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

    pub fn password_matches(self: &Self, hashed_password: &String) -> bool {
        self.password.eq(hashed_password)
    }

    pub async fn add_to_db(db: &Db, user: &User) -> Result<(), model::Error> {
        let bs = bson::to_bson(&user).map_err(|_| model::Error::BsonError)?;
        let document = bs.as_document().unwrap();
        let userdb = db.database("voodoofist").collection::<mongodb::bson::Document>("user");
        userdb.insert_one(document.to_owned(), None).await.map_err(|_| model::Error::DbError("Failed inserting item"))?;
        Ok(())
    }

    pub async fn get_by_email(db: &Db, email: &String) -> Result<User, model::Error> {
        let db = db
            .database("voodoofist")
            .collection::<mongodb::bson::Document>("user");

        let document: Document = db.find_one(
                doc!{"email": email.as_str()}, 
                None
            ).await
            .map_err(|_| model::Error::NoUserWithSuchEmail)?
            .ok_or(model::Error::NoUserWithSuchEmail)?;
    
        let user: User = User::from_document(document).ok_or(model::Error::BsonError)?;
        Ok(user)
    }
}
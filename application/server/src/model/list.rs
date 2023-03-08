use bson::oid::ObjectId;
use serde::{Serialize, Deserialize};
use super::user::User;
use rand::random;
use super::Error;

use super::db::init_db;
use super::user;

#[derive(Serialize, Deserialize, Debug)]
pub struct Rgb {
    r: u8,
    g: u8,
    b: u8
}

impl Rgb {
    pub fn new(r: u8, g: u8, b: u8) -> Rgb {
        Rgb{r, g, b}
    }
    pub fn random() -> Rgb {
        Self::new(random::<u8>(), random::<u8>(), random::<u8>())
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct List {
    id: Option<ObjectId>,
    owner_id: ObjectId,
    title: String,
    color: Rgb
}

impl List {
    pub fn new(owner: &User, title: &String) -> Result<List, Error> {
        Ok(List {
            id: None,
            owner_id: owner.id().ok_or(Error::InvalidUserID)?.clone(),
            title: title.clone(),
            color: Rgb::random()
        })
    }
}
use bson::{doc, bson, oid::ObjectId, Bson, Document};
use serde::{Serialize, Deserialize};
use super::{Db, Error, db, objectid_from_str, objectid_from_str_raw, from_document};
use rand::random;
use crate::error::BsonError;
use super::DATABASE;

const COLLECTION: &'static str = "list";

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
        Self::new(
            random::<u8>(),
            random::<u8>(),
            random::<u8>(),
        )
    }
}

impl Clone for Rgb {
    fn clone(self: &Self) -> Rgb {
        Self::new(self.r, self.g, self.b)
    }
}

impl From<Rgb> for Bson {
    fn from(rgb: Rgb) -> Self {
        bson!({
            "r": rgb.r as u32,
            "g": rgb.g as u32,
            "b": rgb.b as u32
        })
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct List {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<ObjectId>,
    owner_id: ObjectId,
    title: String,
    color: Rgb
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ListPatch {
    _id: ObjectId,
    title: Option<String>,
    color: Option<Rgb>,
}

impl List {
    pub fn new(owner_oid: &str, title: &String) -> Result<List, Error> {
        Ok(List {
            id: None,
            owner_id: objectid_from_str(owner_oid)?,
            title: title.clone(),
            color: Rgb::random()
        })
    }

    pub async fn add_to_db(db: &Db, list: &List) -> Result<(), Error>{
        let document = bson::to_bson(&list)
            .map_err(|err| BsonError::from(err))?
            .as_document().ok_or(BsonError::ConversionError)?
            .to_owned();

        let listdb = db
            .database(DATABASE)
            .collection::<mongodb::bson::Document>(COLLECTION);

        listdb.insert_one(document, None).await
            .map_err(|_| Error::DbError("insert", format!("{:?}", list)))?;

        Ok(())
    }

    pub async fn get_users_lists(db: &Db, uid: &str) -> Result<Vec<List>, Error> {
        let oid = objectid_from_str(uid)?;
        let filter = doc!{"owner_id": oid};

        let documents = db::get_all_in_vec(db, filter, None, COLLECTION).await?;
        let mut lists: Vec<List> = vec![];
        for doc in documents {
            let list = from_document(doc.clone())?;
            lists.push(list);
        }

        Ok(lists)
    }

    pub async fn get_users_list(db: &Db, uid: &str, list_id: &str) -> Result<Option<List>, Error> {
        let oid = objectid_from_str(uid)?;
        let list_id = objectid_from_str_raw(list_id)?;
        let filter = doc!{
            "_id": list_id,
            "owner_id": oid
        };

        let db = db
            .database(DATABASE)
            .collection::<List>(COLLECTION);

        let list_opt = db.find_one(filter.clone(), None).await
            .map_err(|_| Error::DbError("find", format!("{:?}", filter)))?;

        Ok(list_opt)
    }

    pub async fn search(db: &Db, uid: &str, title: &str) -> Result<Vec<List>, Error> {
        let oid = objectid_from_str(uid)?;
        let filter = doc! {
            "owner_id": oid,
            "title": title,
        };

        let documents = db::get_all_in_vec(db, filter, None, COLLECTION).await?;
        let mut lists: Vec<List> = vec![];
        for doc in documents {
            let list = from_document(doc.clone())?;
            lists.push(list);
        }

        Ok(lists)
    }

    pub async fn delete(db: &Db, oid: &ObjectId) -> Result<u64, Error> {
        let filter = doc! {"_id": oid};

        let db = db
            .database(DATABASE)
            .collection::<Document>(COLLECTION);
        let count = db.delete_one(filter, None).await
            .map_err(|_| Error::DbError("delete", format!("{:?}", oid)))?
            .deleted_count;

        Ok(count)
    }

    pub async fn update(db: &Db, patch: &ListPatch) -> Result<u64, Error>{
        let mut update = Document::new();

        if let Some(new_title) = &patch.title {
            update.insert("title", new_title);
        }
        if let Some(new_color) = patch.color.clone() {
            update.insert("color", new_color);
        }
        
        let update = doc! {"$set": update};
        let filter = doc! {"_id": patch._id};
        let db = db
            .database(DATABASE)
            .collection::<Document>(COLLECTION);

        let count = db.update_one(filter, update, None).await
                .map_err(|_| Error::DbError("patch", format!("{:?}", patch)))?
                .modified_count;
        
        Ok(count)
    }
}
use std::sync::Arc;

use bson::{Document, oid::ObjectId, doc};
use super::{Error, Db, db, from_document, BsonError};
use serde::{Serialize, Deserialize};
use super::DATABASE;

const COLLECTION: &'static str = "item";

#[derive(Serialize, Deserialize, Debug)]
pub struct TodoItem {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<ObjectId>,
    list_id: ObjectId,
    text: String,
    is_done: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TodoItemPatch {
    _id: ObjectId,
    text: Option<String>,
    is_done: Option<bool>,
}

impl TodoItem {
    pub fn create(owning_list_id: ObjectId, text: &str) -> TodoItem {
        TodoItem {
            id: None,
            list_id: owning_list_id.clone(),
            text: String::from(text),
            is_done: false,
        }
    }

    pub async fn add_to_db(db: Arc<Db>, item: &TodoItem) -> Result<(), Error> {
        let document = bson::to_bson(&item)
            .map_err(|err| BsonError::from(err))?
            .as_document().ok_or(BsonError::ConversionError)?
            .to_owned();

        let db = db
            .database(DATABASE)
            .collection(COLLECTION);

        db.insert_one(document, None).await
            .map_err(|_| Error::DbError("insert", format!("{:?}", item)))?;
        Ok(())
    }

    pub async fn get_items_from_list(db: Arc<Db>, owning_list_id: &ObjectId) -> Result<Vec<TodoItem>, Error> {
        let filter = doc!{"list_id": owning_list_id};
        
        let documents = db::get_all_in_vec(&db, filter, None, COLLECTION).await?;
        let mut items: Vec<TodoItem> = vec![];
        for doc in documents {
            let list = from_document(doc.clone())?;
            items.push(list);
        }

        Ok(items)
    }

    pub async fn delete(db: Arc<Db>, item_oid: &ObjectId) -> Result<u64, Error> {
        let filter = doc! {
            "_id": item_oid
        };

        let db = db
            .database(DATABASE)
            .collection::<Document>(COLLECTION);
        let count = db.delete_one(filter, None).await
            .map_err(|_| Error::DbError("delete", format!("{:?}", item_oid)))?
            .deleted_count;

        Ok(count)
    }

    pub async fn delete_all_from_list(db: Arc<Db>, owning_list_id: &ObjectId) -> Result<u64, Error> {
        let filter = doc! {
            "list_id": owning_list_id
        };

        let dbc = db
            .database(DATABASE)
            .collection::<TodoItem>(COLLECTION);
        let mut cursor = dbc.find(filter, None).await
            .map_err(|_| Error::DbError("delete items", format!("{:?}", owning_list_id)))?;
        let mut counter: u64 = 0;
        while cursor.advance().await
            .map_err(|_| Error::DbError("advance cursor deleting list", format!("{:?}", owning_list_id)))? 
        {
            let oid = cursor.current()
                .get_object_id("_id")
                .map_err(|err| BsonError::from(err))?;

            counter += Self::delete(db.clone(), &oid).await?;
        }

        Ok(counter)
    }

    pub async fn update(db: Arc<Db>, patch: &TodoItemPatch) -> Result<u64, Error> {
        let mut update = Document::new();
        if let Some(new_text) = &patch.text {
            update.insert("text", new_text);
        }
        if let Some(state) = patch.is_done {
            update.insert("is_done", state);
        }
        
        let update = doc! {"$set": update};
        let filter = doc! {"_id": patch._id};
        let db = db
            .database(DATABASE)
            .collection::<Document>(COLLECTION);

        let count = db.update_one(filter, update, None).await
                .map_err(|_| Error::DbError("patch", format!("{:?}", patch)))?.modified_count;
        
        Ok(count)
    }
}
use std::sync::Arc;

use bson::{Document, Bson, oid::ObjectId, doc};
use futures::TryFutureExt;
use super::{Error, list::List, Db, objectid_from_str, db, from_document};
use serde::{Serialize, Deserialize};

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
        let document = bson::to_bson(&item).map_err(|_| Error::BsonError)?
            .as_document().ok_or(Error::BsonError)?
            .to_owned();
        let db = db
            .database("voodoofist")
            .collection("item");
        db.insert_one(document, None).await
            .map_err(|_| Error::DbError("Couldn't add item"))?;
        Ok(())
    }

    pub async fn get_items_from_list(db: Arc<Db>, owning_list_id: &ObjectId) -> Result<Vec<TodoItem>, Error> {
        let filter = doc!{"list_id": owning_list_id};
        let documents = db::get_all_in_vec(&db, filter, None, &String::from("item")).await?;
        let mut items: Vec<TodoItem> = vec![];
        for doc in documents {
            let list = from_document(doc.clone())?;
            items.push(list);
        }

        Ok(items)
    }

    pub async fn delete(db: Arc<Db>, item_oid: &ObjectId) -> Result<u64, Error> {
        let db = db
            .database("voodoofist")
            .collection::<Document>("item");
        let filter = doc! {"_id": item_oid};
        let result = db.delete_one(filter, None).await
            .map_err(|_| Error::DbError("Couldn't delete item"))?;
        Ok(result.deleted_count)
    }

    pub async fn delete_all_from_list(db: Arc<Db>, owning_list_id: &ObjectId) -> Result<u64, Error> {
        let filter = doc! {"list_id": owning_list_id};
        let dbc = db
            .database("voodoofist")
            .collection::<TodoItem>("item");
        let mut cursor = dbc.find(filter, None).await
            .map_err(|_| Error::DbError("Could't fetch items"))?;
        let mut counter: u64 = 0;

        while cursor.advance().await.map_err(|_| Error::DbError("Couldn't advance cursor"))? {
            let oid = cursor.current().get_object_id("_id").map_err(|_| Error::BsonError)?;
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
            .database("voodoofist")
            .collection::<Document>("item");

        let count = db.update_one(filter, update, None).await
                .map_err(|_| Error::DbError("Couldn't patch list"))?.modified_count;
        
        Ok(count)
    }
}
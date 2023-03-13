use std::sync::Arc;
use bson::oid::ObjectId;
use serde::Deserialize;
use serde_json::json;
use warp::{Filter, Rejection, reply::Json};
use crate::model::{Db, todo_item::{TodoItem, TodoItemPatch}};
use super::{with_auth, json_response, Error};

#[derive(Deserialize, Debug)]
struct NewItemBody {
    list_oid: ObjectId,
    text: String,
}

pub fn todo_item_paths(db: Arc<Db>) -> impl Filter<Extract = (impl warp::Reply,), Error = Rejection> + Clone {
    let with_db = warp::any().map(move || db.clone());
    let common = with_db.clone().and(with_auth());

    let get_lists_items = warp::path("items")
        .and(warp::post())
        .and(common.clone())
        .and(warp::body::json())
        .and_then(get_items_handle);

    let add_item = warp::path("item").and(warp::path::end())
        .and(warp::post())
        .and(common.clone())
        .and(warp::body::json())
        .and_then(add_item_handle);

    let delete_item = warp::path("item").and(warp::path::end())
        .and(warp::delete())
        .and(common.clone())
        .and(warp::body::json())
        .and_then(delete_item_handle);

    let update_item = warp::path("item").and(warp::path::end())
        .and(warp::patch())
        .and(common.clone())
        .and(warp::body::json())
        .and_then(update_item_handle);

    get_lists_items.or(add_item).or(delete_item).or(update_item)
}

async fn get_items_handle(db: Arc<Db>, _oid: String, list_oid: ObjectId) -> Result<Json, Rejection> {
    let items = TodoItem::get_items_from_list(db.clone(), &list_oid).await.map_err(|_| Error::InnerError)?;

    let content = json!({"items": items});
    json_response(&content)
}

async fn add_item_handle(db: Arc<Db>, _oid: String, body: NewItemBody) -> Result<Json, Rejection> {
    let item = TodoItem::create(body.list_oid.clone(), &body.text);

    TodoItem::add_to_db(db.clone(), &item).await.map_err(|_| Error::InnerError)?;

    let content = json!({"Inserted item": body.text});
    json_response(&content)
}

async fn delete_item_handle(db: Arc<Db>, _oid: String, item_oid: ObjectId) -> Result<Json, Rejection> {
    let count = TodoItem::delete(db.clone(), &item_oid).await.map_err(|_| Error::InnerError)?;

    let content = json!({"Deleted item": item_oid.to_string(), "count": count});
    json_response(&content)
}

async fn update_item_handle(db: Arc<Db>, _oid: String, patch: TodoItemPatch) -> Result<Json, Rejection> {
    let count = TodoItem::update(db.clone(), &patch).await
        .map_err(|_| Error::InnerError)?;

    let content = json!({"Updated item": patch, "count": count});
    json_response(&content)
}
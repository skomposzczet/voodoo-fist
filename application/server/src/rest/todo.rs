use std::collections::HashMap;
use std::sync::Arc;
use bson::oid::ObjectId;
use warp::{reply::Json, Rejection, Filter};
use crate::model::list::ListPatch;
use crate::model::todo_item::TodoItem;
use crate::model::{Db, list::List};
use crate::rest::{Error, json_response, with_auth};
use serde_json::json;

pub fn todo_list_paths(db: Arc<Db>) -> impl Filter<Extract = (impl warp::Reply,), Error = Rejection> + Clone {
    let with_db = warp::any().map(move || db.clone());
    let common = with_db.clone().and(with_auth());

    let get_lists = warp::path("lists").and(warp::path::end())
        .and(warp::get())
        .and(common.clone())
        .and_then(get_lists_handle);

    let new_list = warp::path("list").and(warp::path::end())
        .and(warp::post())
        .and(common.clone())
        .and(warp::body::json())
        .and_then(post_list_handle);

    let delete_list = warp::path("list").and(warp::path::end())
        .and(warp::delete())
        .and(common.clone())
        .and(warp::body::json())
        .and_then(delete_handle);

    let patch_list = warp::path("list").and(warp::path::end())
        .and(warp::patch())
        .and(common.clone())
        .and(warp::body::json())
        .and_then(patch_handle);

    get_lists.or(new_list).or(delete_list).or(patch_list)
}

async fn get_lists_handle(db: Arc<Db>, oid: String) -> Result<Json, Rejection> {
    let lists = List::get_users_lists(&db, &oid).await
        .map_err(|_| Error::InnerError)?;

    let response = json!({"lists": lists});
    json_response(&response)
}

async fn post_list_handle(db: Arc<Db>, oid: String, body: HashMap<String, String>) -> Result<Json, Rejection> {
    let title = body.get("title")
        .ok_or(Error::BodyError("Expected key: title"))?;

    let list = List::new(&oid, title)
        .map_err(|_| Error::InnerError)?;
    List::add_to_db(&db, &list).await
        .map_err(|_| Error::InnerError)?;

    let content = json!({"Inserted list": title});
    json_response(&content)
}

async fn delete_handle(db: Arc<Db>, _oid: String, list_oid: ObjectId) -> Result<Json, Rejection> {
    let items_count = TodoItem::delete_all_from_list(db.clone(), &list_oid).await
        .map_err(|_| Error::InnerError)?;
    let count = List::delete(&db, &list_oid).await
        .map_err(|_| Error::InnerError)?;
    let content = json!({"Deleted list": list_oid, "Count": count, "Deleted items count": items_count});
    json_response(&content)
}

async fn patch_handle(db: Arc<Db>, _oid: String, list: ListPatch) -> Result<Json, Rejection> {
    let count = List::update(&db, &list).await
        .map_err(|_| Error::InnerError)?;
    let content = json!({"Patched list": list, "Count": count});
    json_response(&content)
}
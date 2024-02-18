use mongodb::bson::{doc, Document};
use mongodb::Collection;
use warp::http;
use crate::{Id, Store};

pub async fn delete_grocery_list_item(
    id: Id,
    store: Store,
    table: Collection<Document>
) -> Result<impl warp::Reply, warp::Rejection> {
    store.grocery_list.write().remove(&id.name);
    table.delete_one(
        doc! {
            "name": &id.name
        }, None
    ).await.unwrap();
    Ok(warp::reply::with_status(
        "Removed item from grocery list",
        http::StatusCode::OK,
    ))
}
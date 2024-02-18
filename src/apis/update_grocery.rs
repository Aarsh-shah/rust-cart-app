use mongodb::bson::{doc, Document};
use mongodb::Collection;
use mongodb::options::UpdateOptions;
use warp::http;
use crate::{Item, Store};

pub async fn update_grocery_list(
    item: Item,
    store: Store,
    table: Collection<Document>
) -> Result<impl warp::Reply, warp::Rejection> {
    let name = item.name;
    store.grocery_list.write().insert(name.to_owned(), item.quantity);
    let filter = doc! {"name": name.clone()};
    let update = doc! {"$set": doc!{ "quantity": item.quantity }};
    let options = UpdateOptions::builder().upsert(true).build();
    table.update_one(
        filter, update, options
    ).await.expect("failed to update database");
    Ok(warp::reply::with_status(
        "Updated value of items from the grocery list",
        http::StatusCode::CREATED,
    ))
}
use mongodb::bson::{doc, Document};
use mongodb::Collection;
use mongodb::options::UpdateOptions;
use warp::http;
use crate::{Item, Store};

pub async fn add_to_existing_grocery_list(
    item: Item,
    store: Store,
    table: Collection<Document>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let name = item.name;
    let filter = doc! {"name": name.to_owned()};
    let find_existing_key = table.find_one(
        doc! {
            "name": name.clone()
        }, None
    ).await.unwrap();
    let mut previous_quantity = 0;
    if find_existing_key != None {
        previous_quantity = find_existing_key.unwrap().get("quantity").unwrap().as_i32().unwrap();
    }
    let new_quantity = previous_quantity + item.quantity;
    let update = doc! {"$set": doc!{ "quantity": new_quantity }};
    let options = UpdateOptions::builder().upsert(true).build();

    table.update_one(
        filter, update, options
    ).await.expect("failed to update database");
    store.grocery_list.write().insert(name, new_quantity);
    Ok(warp::reply::with_status(
        "Added to grcoery list",
        http::StatusCode::CREATED,
    ))
}
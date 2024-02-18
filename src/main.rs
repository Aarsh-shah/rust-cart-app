mod redis;
mod getprice;
mod apis {
    pub mod add_to_existing_grocery;
    pub mod delete_grocery_item;
    pub mod get_grocery_item;
    pub mod get_grocery_list;
    pub mod get_nutrition;
    pub mod update_grocery;
}

mod chatgpt {
    pub mod getrecipe;
}

use warp::{Filter};
use parking_lot::RwLock;
use std::collections::HashMap;
use std::sync::Arc;

use serde::{Serialize, Deserialize};
use mongodb::{bson::{Document, doc}, Client, Collection};
use crate::chatgpt::getrecipe::get_api_resp;
use crate::redis::{get_redis_pool};

type Items = HashMap<String, i32>;

#[derive(Debug, Deserialize, Serialize, Clone)]
struct Id {
    name: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct Item {
    name: String,
    quantity: i32,
}

#[derive(Clone)]
struct Store {
  grocery_list: Arc<RwLock<Items>>
}

impl Store {
    fn new() -> Self {
        Store {
            grocery_list: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

fn delete_json() -> impl Filter<Extract = (Id,), Error = warp::Rejection> + Clone {
    // When accepting a body, we want a JSON body
    // (and to reject huge payloads)...
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

fn post_json() -> impl Filter<Extract = (Item,), Error = warp::Rejection> + Clone {
    // When accepting a body, we want a JSON body
    // (and to reject huge payloads)...
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

// fn redis_start_server() {
//     let output = Command::new("redis-server")
//         .output()
//         .expect("failed to execute process");
//     println!("{}", String::from_utf8_lossy(&output.stdout));
// }

#[tokio::main]
async fn main() -> mongodb::error::Result<()> {

    let uri = env!("MONGO_URI");
    // Create a new client and connect to the server
    let client = Client::with_uri_str(uri).await?;
    let backend_db = client.database("rust-backend");
    let my_table: Collection<Document> = backend_db.collection("cart");
    let table_filter = warp::any().map(move || my_table.clone());

    let store = Store::new();
    let pool = get_redis_pool().await;
    let store_filter = warp::any().map(move || store.clone());
    getprice::get_product_data(3017624010701).await;

    let add_items = warp::post()
        .and(warp::path("v1"))
        .and(warp::path("groceries"))
        .and(warp::path::end())
        .and(post_json())
        .and(store_filter.clone())
        .and(table_filter.clone())
        .and_then(apis::add_to_existing_grocery::add_to_existing_grocery_list);

    let get_items = warp::get()
        .and(warp::path("v1"))
        .and(warp::path("groceries"))
        .and(warp::path::end())
        .and(store_filter.clone())
        .and_then(apis::get_grocery_list::get_grocery_list);

    let get_item = warp::get()
        .and(warp::path("v1"))
        .and(warp::path("groceries"))
        .and(warp::path::param::<String>())
        .and(warp::path::end())
        .and(store_filter.clone())
        .and(table_filter.clone())
        .and_then(apis::get_grocery_item::get_grocery_item);

    let delete_item = warp::delete()
        .and(warp::path("v1"))
        .and(warp::path("groceries"))
        .and(warp::path::end())
        .and(delete_json())
        .and(store_filter.clone())
        .and(table_filter.clone())
        .and_then(apis::delete_grocery_item::delete_grocery_list_item);


    let update_item = warp::put()
        .and(warp::path("v1"))
        .and(warp::path("groceries"))
        .and(warp::path::end())
        .and(post_json())
        .and(store_filter.clone())
        .and(table_filter.clone())
        .and_then(apis::update_grocery::update_grocery_list);

    let get_energy = warp::get()
        .and(warp::path("v1"))
        .and(warp::path("nutrition"))
        .and(warp::path::param::<i64>())
        .and(warp::path::end())
        .and(warp::any().map(move || Arc::clone(&pool)))
        .and_then(apis::get_nutrition::add_to_redis);

    let get_recipe = warp::get()
        .and(warp::path("v1"))
        .and(warp::path("recipe"))
        .and(warp::path::end())
        .and(store_filter.clone())
        .and_then(get_api_resp);

    let routes = add_items.or(get_items).or(delete_item).or(update_item).or(get_item).or(get_energy).or(get_recipe);

    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
    println!("Started the server");
    Ok(())
}
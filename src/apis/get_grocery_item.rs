use mongodb::bson::{doc, Document};
use mongodb::Collection;
use warp::http;
use crate::Store;

pub(crate) async fn get_grocery_item(
    item: String,
    store: Store,
    table: Collection<Document>
) -> Result<impl warp::Reply, warp::Rejection> {
    let r = store.grocery_list.read().get(&item).cloned();
    if r.is_some() {
        let html_response = format!(r#"<html><body><p style="color:red;">Value for item {}</p><p>{}</p></body></html>"#, &item, r.unwrap());

        Ok(warp::reply::with_status(
            warp::reply::html(html_response),
            http::StatusCode::CREATED,
        ))
    }
    else {
        let find_existing_key = table.find_one(
            doc! {
            "name": &item
        }, None
        ).await.unwrap();
        if find_existing_key.is_some() {
            let quantity = find_existing_key.unwrap().get("quantity").unwrap().as_i32().unwrap();
            store.grocery_list.write().insert(item.clone().to_owned(), quantity.clone().to_owned());
            let html_response = format!(r#"<html><body><p style="color:red;">Value for item {}</p><p>{}</p></body></html>"#, &item, quantity.clone());
            Ok(warp::reply::with_status(
                warp::reply::html(html_response),
                http::StatusCode::CREATED,
            ))
        } else {
            Ok(warp::reply::with_status(
                warp::reply::html(format!("not found {}", &item)),
                http::StatusCode::NOT_FOUND,
            ))
        }
    }
}
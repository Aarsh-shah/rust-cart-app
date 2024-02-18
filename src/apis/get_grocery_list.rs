use crate::Store;

pub(crate) async fn get_grocery_list(
    store: Store,
) -> Result<impl warp::Reply, warp::Rejection> {
    let r = store.grocery_list.read();
    Ok(warp::reply::json(&*r))
}
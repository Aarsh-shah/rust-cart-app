use reqwest::Client;
use serde_json::Value;

pub async fn get_product_data(product_id: i64) -> Value {
    let client = Client::new();
    let url = format!("https://world.openfoodfacts.net/api/v2/product/{}?fields=nutriscore_data", product_id);
    println!("URL: {}", url);
    let res = client.get(url).send().await.unwrap();
    res.status();
    let body = res.json::<Value>().await.unwrap();
    return body;
}
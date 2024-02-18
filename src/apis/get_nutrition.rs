use std::sync::Arc;
use redis::{AsyncCommands, RedisError};
use redis_async_pool::RedisPool;
use warp::http;
use crate::getprice::get_product_data;

pub async fn add_to_redis(
    item: i64,
    pool: Arc<RedisPool>
) -> Result<impl warp::Reply, warp::Rejection> {
    let mut conn = pool.get().await;
    let key_item = format!("\"{}\"", item.to_string());
    let energy_val: Result<Option<i64>, RedisError> = conn.as_mut().expect("REASON").get(key_item).await;
    match energy_val  {
        Ok(value) => {
            if let Some(value) = value {
                return Ok(warp::reply::with_status(
                    format!("Energy Value for item {} is {}", item, value),
                    http::StatusCode::OK,
                ))
            }
        }
        Err(error) => {
            panic!("Error: {:?}", error);
        }
    }
    let product_details = get_product_data(item).await;
    let code = product_details["code"].to_string();
    let energy = product_details["product"]["nutriscore_data"]["energy"].as_i64().unwrap();
    let _result: Result<(), RedisError> = conn.as_mut().expect("REASON").set(code.clone(), energy).await;
    match _result {
        Ok(_) => {
            let energy_val: Result<Option<i64>, RedisError> = conn.as_mut().expect("REASON").get(code).await;
            match energy_val {
                Ok(value) => {
                    if let Some(value) = value {
                        // assert_eq!(value, 2055);
                        println!("Value: {:?}", value);
                    } else {
                        panic!("Key not found");
                    }
                }
                Err(error) => {
                    panic!("Error: {:?}", error);
                }
            }
        }
        Err(err) => {
            eprintln!("Error setting value in Redis: {:?}", err);
            // Handle the case where an error occurred during the set operation
        }
    }

    Ok(warp::reply::with_status(
        format!("Didn't found in redis: New value for code {} is {:?}", item, energy),
        http::StatusCode::CREATED,
    ))

}
use std::sync::Arc;
use redis::{Client, RedisError};
use redis_async_pool::{RedisConnection, RedisConnectionManager, RedisPool};
use redis_async_pool::deadpool::managed::Pool;

pub async fn get_redis_pool() -> Arc<Pool<RedisConnection, RedisError>> {
    return  Arc::new(RedisPool::new(
        RedisConnectionManager::new(Client::open("redis://127.0.0.1/").unwrap(), true, None),
        5,
    ));
}

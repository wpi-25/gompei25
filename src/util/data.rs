use redis::AsyncCommands;
use log::error;
use std::env;

pub async fn get_redis_connection() -> Result<redis::aio::Connection, redis::RedisError> {
    let client = match redis::Client::open(env::var("REDIS_URL").expect("No Redis URL configured - check your .env file")) {
        Ok(client) => client,
        Err(e) => {
           error!("Could not connect to redis: {:?}", e);
           panic!("Could not connect to redis.");
        },
    };

    client.get_async_connection().await
}

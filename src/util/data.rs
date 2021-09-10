use redis::AsyncCommands;
use log::error;
use std::env;

pub fn get_redis_connection() -> Result<redis::Connection, redis::RedisError> {
    let client = match redis::Client::open(env::var("REDIS_URL").expect("No Redis URL configured - check your .env file")) {
        Ok(client) => client,
        Err(e) => {
           error!("Could not connect to redis: {:?}", e);
           panic!("Could not connect to redis.");
        },
    };

    match client.get_connection() {
        Ok(conn) => Ok(conn),
        Err(e) => {
            error!("Error getting redis connection: {:?}", e);
            panic!("Could not connect to redis");
            Err(e)
        }
    }
}

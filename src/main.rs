use std::env;
use std::collections::HashSet;

use serenity::{
    async_trait,
    http::Http,
    framework::{standard::macros::group, StandardFramework},
    model::gateway::Ready,
    prelude::*
};

use log::{error, warn, info};

mod commands;

pub mod util;

pub struct RedisConnection;
impl TypeMapKey for RedisConnection {
    type Value = redis::aio::Connection;
}

use commands::meta::*;
#[group]
#[commands(ping)]
struct Meta;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        info!("Connected as {}", ready.user.name)
    }
}

#[tokio::main]
async fn main() {
    if let Err(e) = dotenv::dotenv() {
        warn!("Could not load .env file, have you set the environment properly? {:?}", e)
    }
    env_logger::init();

    let token = env::var("DISCORD_TOKEN").expect("No token in environment");

    let http = Http::new_with_token(&token);

    let (owners, _bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            owners.insert(info.owner.id);

            (owners, info.id)
        }
        Err(e) => panic!("Could not access application info: {:?}", e),
    };

    let framework = StandardFramework::new().configure(|c| c.owners(owners).prefix(&env::var("DISCORD_PREFIX").expect("No prefix in environment"))).group(&META_GROUP);
    let mut client = Client::builder(&token).framework(framework).event_handler(Handler).await.expect("Could not create discord client");

    {
        let mut data = client.data.write().await;
        let con = match util::data::get_redis_connection().await {
            Ok(red) => red,
            Err(err) => {
                error!("Could not obtain a redis connection: {:?}", err);
                panic!("Obtaining redis connection");
            }
        };

        data.insert::<RedisConnection>(con)
    }

    if let Err(e) = client.start().await {
        error!("Client error: {:?}", e);
    }
}


use std::env;
use std::collections::HashSet;

use serenity::{async_trait, client::bridge::gateway::GatewayIntents, framework::{standard::macros::{group, hook}, StandardFramework}, http::Http, model::channel::Message, model::gateway::Ready, prelude::*, utils::MessageBuilder};

use chrono::prelude::*;

use tracing::{info, error, warn, instrument};

mod commands;


pub mod util;
pub mod errors;

pub struct RedisConnection;
impl TypeMapKey for RedisConnection {
    type Value = redis::Connection;
}

use commands::meta::*;
use commands::leveling::*;
#[group]
#[commands(ping)]
struct Meta;

#[group]
#[commands(rank, levels)]
struct Leveling;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _ctx: Context, ready: Ready) {
        info!("Logged into Discord as {}", ready.user.name)
    }

    #[instrument(skip(self, ctx))]
    async fn message(&self, ctx: Context, msg: Message) {
        if !msg.content.starts_with(&env::var("DISCORD_PREFIX").unwrap()) {
            if !msg.author.bot {
                let mut bot_data = ctx.data.write().await;
                let mut redis_conn = bot_data.get_mut::<RedisConnection>().unwrap();
                match util::leveling::get_user_level(msg.author.id.0, &mut redis_conn) {
                    Ok(data) => {
                        let time_since_last_msg = data.last_msg - Utc::now();
                        if time_since_last_msg.num_minutes() < -1 {
                            let mut new_data = data.clone();
                            new_data.msg_count += 1;
                            new_data.xp += 1;
                            new_data.last_msg = Utc::now();
                           if let Err(e) = util::leveling::set_user_level(msg.author.id.0, &mut redis_conn, new_data) {

                              error!("{:?}", e);
                           }
                        }
                    },
                    Err(e) => {
                        error!("Error computing levels: {:?}", e);
                        return;
                    },
                }
            }
        }
    }
}

#[tokio::main]
#[instrument]
async fn main() {
    if let Err(e) = dotenv::dotenv() {
        warn!("Could not load .env file, have you set the environment properly? {:?}", e)
    }

    tracing_subscriber::fmt::init();

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

    let framework = StandardFramework::new()
        .configure(|c|
            c.owners(owners)
            .prefix(&env::var("DISCORD_PREFIX").expect("No prefix in environment"))
        )
        .group(&META_GROUP)
        .group(&LEVELING_GROUP)
        .help(&HELP);
    let mut client = Client::builder(&token)
        .framework(framework)
        .event_handler(Handler)
        .intents(GatewayIntents::all())
        .await
        .expect("Could not create discord client");

    {
        let mut data = client.data.write().await;
        let con = match util::data::get_redis_connection() {
            Ok(red) => red,
            Err(err) => {
                error!("Could not obtain a redis connection: {:?}", err);
                panic!("Obtaining redis connection");
            }
        };

        data.insert::<RedisConnection>(con)
    }

    info!("Starting client");
    if let Err(e) = client.start().await {
        error!("Client error: {:?}", e);
    }
}

#[hook]
#[instrument]
async fn before(_: &Context, _: &Message, _: &str) -> bool {
    true
}
